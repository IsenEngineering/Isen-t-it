use aeronet::transport::{lane::LaneIndex, Transport};
use bevy::{prelude::*, utils::{hashbrown::HashMap, Instant}};
use bincode;
use isent_it::network::{ClientToServer, ClientsConnection, Player, ServerToClient};

pub fn recv(
    mut commands: Commands,
    mut clients: Query<(&mut Transport, Entity, Option<&mut Player>)>,
) {
    for (mut transport, entity, player) in clients.iter_mut() {
        let mut updates: Vec<ClientToServer> = Vec::new();
        for packet in transport.recv.msgs.drain() {
            match packet.lane {
                // Client connects
                LaneIndex(0) => {
                    if !player.is_none() {
                        warn!(
                            "{} tried to introduce but already introduced", 
                            entity.to_bits().to_string()
                        );
                        continue;
                    } 

                    let ply = match bincode::deserialize::<Player>(
                        packet.payload.as_slice()) {
                        Ok(p) => p,
                        Err(e) => {
                            warn!("couldn't deserialize Player: {e}");
                            continue
                        }
                    };

                    commands.entity(entity).insert(ply);
                },
                // Client moves
                LaneIndex(1) => {
                    let update = match bincode::deserialize::<ClientToServer>(
                        packet.payload.as_slice()) {
                        Ok(u) => u,
                        Err(e) => {
                            warn!("couldn't deserialize ClientToServer: {e}");
                            continue
                        }
                    };
        
                    updates.push(update);
                    // On stocke les mise à jours pour 
                    // les envoyer toutes en même temps.
                },
                _ => {
                    warn!("Un message s'est perdu! {:?}", packet);
                }
            }
        }
        for _ in transport.recv.acks.drain() {}

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

pub fn send_changes(
    players: Query<(&Player, Entity), Changed<Player>>,
    mut clients: Query<(&mut Transport, Entity)>) {
    if players.is_empty() {
        return
    }
    let mut updates: ServerToClient = HashMap::new();

    for (player, entity) in players.iter() {
        let id = entity.to_bits().to_string();
        updates.insert(id, player.position);
    }

    for (mut client, entity) in clients.iter_mut() {
        let mut local_updates = updates.clone();
        let local_name = entity.to_bits().to_string();
        if local_updates.contains_key(&local_name) {
            local_updates.remove(&local_name);
        }
        client.send.push(
            LaneIndex(2), 
            bincode::serialize(&local_updates).unwrap().into(),
            Instant::now()
        );
    }
    info!("{} new updates sent", updates.len());
}

pub fn send_connections(
    players: Query<(&Player, Entity), Added<Player>>,
    mut clients: Query<(&mut Transport, Entity)>) {
    if players.is_empty() {
        return
    }
    let mut updates: ClientsConnection = HashMap::new();

    for (player, entity) in players.iter() {
        let id = entity.to_bits().to_string();
        updates.insert(id, player.clone());
    }

    for (mut client, entity) in clients.iter_mut() {
        let mut local_updates = updates.clone();
        let local_name = entity.to_bits().to_string();
        if local_updates.contains_key(&local_name) {
            local_updates.remove(&local_name);
        }
        client.send.push(
            LaneIndex(0), 
            bincode::serialize(&local_updates).unwrap().into(),
            Instant::now()
        );
    }
    info!("{} new connections sent", updates.len());
}