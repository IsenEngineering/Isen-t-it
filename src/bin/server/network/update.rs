use aeronet::transport::{lane::LaneIndex, Transport};
use bevy::{prelude::*, utils::{hashbrown::HashMap, Instant}};
use bincode;
use isent_it::network::{ClientToServer, ClientsConnection, Player, ServerToClient};

// Gères les messages entrants
pub fn recv(
    mut commands: Commands,

    // Tous les joueurs connectés
    mut clients: Query<(&mut Transport, Entity, Option<&mut Player>)>,
) {
    for (mut transport, entity, player) in clients.iter_mut() {
        // Les modifications émisent par un utilisateur
        let mut updates: Vec<ClientToServer> = Vec::new();

        // Les différents messages récupérés
        for packet in transport.recv.msgs.drain() {

            // Chaque ligne d'envoie corresponds à une action 
            // pour éviter des erreurs de "decompositions"
            match packet.lane {
                // Un joueur se présente,
                LaneIndex(0) => {
                    // S'il n'y a pas de joueur attaché à la connexion.
                    if !player.is_none() {
                        warn!(
                            "{} a essayé de se présenter mais l'est déjà!", 
                            entity.to_bits().to_string()
                        );
                        continue;
                    } 

                    // On décompose les "Bytes" en données utilisable
                    let ply = match bincode::deserialize::<Player>(
                        packet.payload.as_slice()) {
                        // Cas de réussite
                        Ok(p) => p,
                        // Erreur
                        Err(e) => {
                            warn!("'Player' indecomposable: {e}");
                            continue
                        }
                    };

                    // On attache à la connexion, le joueur
                    commands.entity(entity).insert(ply);
                },
                // Lorsqu'un joueur bouge
                LaneIndex(1) => {
                    let update = match bincode::deserialize::<ClientToServer>(
                        packet.payload.as_slice()) {
                        Ok(u) => u,
                        Err(e) => {
                            warn!("'ClientToServer' indecomposable: {e}");
                            continue
                        }
                    };
        
                    updates.push(update);
                    // On stocke les mise à jours pour 
                    // les envoyer toutes en même temps.
                },
                // On n'attends aucun autre message.
                _ => {
                    warn!("Un message s'est perdu! {:?}", packet);
                }
            }
        }

        // les "ack" attestants de la bonne récéption des messages envoyés.
        for _ in transport.recv.acks.drain() {}

        // S'il y a eu des modifications, 
        // On les effectue, un autre système détectera les changements 
        // et enverra les modifications aux autres clients.
        if updates.len() > 0 {
            match player {
                Some(mut p) => {
                    let last_update = updates.last().unwrap();
                    p.position = *last_update;
                },
                None => {
                    continue;
                }
            }
        }
    }
}

// Ce système écoute les modifications des joueurs et les envoie aux autres joueurs
pub fn send_changes(
    // Les joueurs modifiées, on les trouve avec le filtre Changed<T>
    players: Query<(&Player, Entity), Changed<Player>>,
    mut clients: Query<(&mut Transport, Entity)>) {

    // Le cas où il n'y a aucunes modifications
    if players.is_empty() {
        return
    }

    // Les modifications sous forme de clé/valeur
    let mut updates: ServerToClient = HashMap::new();

    // On remplie le tableau des modifications
    for (player, entity) in players.iter() {
        // L'identifiant de l'entité sert d'identifiant pour le joueur
        let id = entity.to_bits().to_string();
        updates.insert(id, player.position);
    }

    // On envoie à tous les clients les modifications sauf leurs propres mouvements.
    for (mut client, entity) in clients.iter_mut() {
        // On copie le tableau pour le modifier dans ce scope
        let mut local_updates = updates.clone();
        
        // On prends l'identifiant de ce client.
        let local_name = entity.to_bits().to_string();

        // S'il y a une modification à son nom
        if local_updates.contains_key(&local_name) {
            // On la retire
            local_updates.remove(&local_name);
        }

        
        client.send.push(
            LaneIndex(2), 
            bincode::serialize(&local_updates).unwrap().into(),
            Instant::now()
        );
        debug!("{} updates sent to {}", local_updates.len(), local_name);
    }
}

// Ce système envoie à tous les utilisateurs les 
// informations sur les joueurs qui se connectent.
pub fn send_connections(
    players: Query<(&Player, Entity), Added<Player>>,
    mut clients: Query<(&mut Transport, Entity)>) {

    // On fait rien si aucun joueur se connecte.
    if players.is_empty() {
        return
    }
    let mut updates: ClientsConnection = HashMap::new();

    for (player, entity) in players.iter() {
        let id = entity.to_bits().to_string();
        updates.insert(id, player.clone());
    }

    for (mut client, entity) in clients.iter_mut() {
        // On veille à ne pas envoie un nouveau joueur à lui même

        let mut local_updates = updates.clone();

        // Nom de l'utilisateur en question
        let local_name = entity.to_bits().to_string();
        if local_updates.contains_key(&local_name) {
            local_updates.remove(&local_name);
        }
        client.send.push(
            LaneIndex(0), 
            bincode::serialize(&local_updates).unwrap().into(),
            Instant::now()
        );
        debug!("{} new connections sent to {}", local_updates.len(), local_name);
    }
}