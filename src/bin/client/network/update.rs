use aeronet::transport::lane::LaneIndex;
use aeronet::transport::Transport;
use bevy::{prelude::*, utils::Instant};
use isent_it::joueur::composants::{Joueur, JoueurPrincipal};
use isent_it::joueur::spawn_player;
use isent_it::network::{ClientDeconnection, ClientToServer, ClientsConnection, ServerToClient};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum ClientNetworkSet {
    Send,
    Recv
}

// On envoie nos mouvements
pub fn send(
    mut transports: Query<&mut Transport>,
    positions: Query<&Transform, (With<JoueurPrincipal>, Changed<Transform>)>
) {
    for mut transport in transports.iter_mut() {
        let position = match positions.get_single() {
            Ok(p) => p,
            _ => return
        };
    
        // On prends la position qui a changé
        let update: ClientToServer = position.translation;
    
        // On l'envoie sur une ligne UDP
        transport.send.push(
            LaneIndex(1), 
            bincode::serialize(&update).unwrap().into(),
            Instant::now()
        );
    }
}

// Récéption des messages du serveur
pub fn recv(
    mut commands: Commands,
    mut players: Query<(&mut Transform, &Name, Entity), With<Joueur>>,
    mut transports: Query<&mut Transport>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for mut transport in transports.iter_mut() {
        'packets: for packet in transport.recv.msgs.drain() {
            match packet.lane {
                // Lorsqu'un nouveau joueur se connecte
                LaneIndex(0) => {
                    let connections: bevy::utils::hashbrown::HashMap<String, isent_it::network::Player> = match bincode::deserialize::<ClientsConnection>(
                        &packet.payload) {
                        Ok(p) => p,
                        Err(e) => {
                            warn!("can't deserialise ClientsConnection: {e}");
                            continue 'packets;
                        }
                    };

                    // On instancie les joueurs
                    for (name, player) in connections.iter() {
                        let player = spawn_player(
                            &mut commands, 
                            &asset_server, 
                            &mut texture_atlas_layouts, 
                            player.skin, 
                            player.position,
                        );
        
                        commands.entity(player).insert(Name::new(name.to_string()));
                    }
                },
                // Lorsque des autres joueurs se déplacent
                LaneIndex(2) => {
                    let mut positions = match bincode::deserialize::<ServerToClient>(&packet.payload) {
                        Ok(p) => p,
                        Err(e) => {
                            warn!("can't deserialise Vec3: {e}");
                            continue 'packets;
                        }
                    };

                    'players_position: for (mut player, name, _) in players.iter_mut() {
                        // Si l'utilisateur est dans le tableau des modifications
                        // On modifie sa position
                        if !positions.contains_key(name.as_str()) {
                            continue 'players_position;
                        }
        
                        let position = positions.remove(name.as_str()).unwrap();
                        player.translation = position;
                    }
                },
                // Lorsque d'autres joueurs se deconnectent 
                LaneIndex(3) => {
                    let player = match bincode::deserialize::<ClientDeconnection>(&packet.payload) {
                        Ok(d) => d,
                        Err(e) => {
                            warn!("can't deserialise String: {e}");
                            continue 'packets;
                        }
                    };

                    // On supprime les joueurs en question
                    for (_, name, entity) in players.iter_mut() {
                        if player.eq(name.as_str()) {
                            commands.entity(entity).despawn();
                        }
                    }
                },
                // On n'attends aucun autre message.
                _ => {
                    warn!("Un message s'est perdu! {:?}", packet);
                }
            }
        }
    
        for _ in transport.recv.acks.drain() {}
    }
    
}