use aeronet::transport::lane::LaneIndex;
use aeronet::transport::Transport;
use bevy::{prelude::*, utils::Instant};
use isent_it::joueur::composants::{Joueur, JoueurPrincipal};
use isent_it::joueur::spawn_player;
use isent_it::network::{ClientDeconnection, ClientToServer, ClientsConnection, ServerToClient};

pub fn send(
    mut transports: Query<&mut Transport>,
    positions: Query<&Transform, (With<JoueurPrincipal>, Changed<Transform>)>
) {
    for mut transport in transports.iter_mut() {
        let position = match positions.get_single() {
            Ok(p) => p,
            _ => return
        };
    
        let update: ClientToServer = position.translation;
    
        transport.send.push(
            LaneIndex(1), 
            bincode::serialize(&update).unwrap().into(),
            Instant::now()
        );
        info!("position sent on lane 1");
    }
}

pub fn recv(
    mut commands: Commands,
    mut players: Query<(&mut Transform, &Name, Entity), With<Joueur>>,
    mut transports: Query<&mut Transport>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for mut transport in transports.iter_mut() {
        for packet in transport.recv.msgs.drain() {
            match packet.lane {
                // A player connects
                LaneIndex(0) => {
                    let connections = match bincode::deserialize::<ClientsConnection>(
                        &packet.payload) {
                        Ok(p) => p,
                        Err(e) => {
                            warn!("can't deserialise ClientsConnection: {e}");
                            continue;
                        }
                    };

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
                // Other players' position updates
                LaneIndex(2) => {
                    let mut positions = match bincode::deserialize::<ServerToClient>(&packet.payload) {
                        Ok(p) => p,
                        Err(e) => {
                            warn!("can't deserialise Vec3: {e}");
                            continue;
                        }
                    };

                    for (mut player, name, _) in players.iter_mut() {
                        if !positions.contains_key(name.as_str()) {
                            continue;
                        }
        
                        let position = positions.remove(name.as_str()).unwrap();
                        player.translation = position;
                    }
                },
                // Other players' deconnection
                LaneIndex(3) => {
                    let player = match bincode::deserialize::<ClientDeconnection>(&packet.payload) {
                        Ok(d) => d,
                        Err(e) => {
                            warn!("can't deserialise String: {e}");
                            continue;
                        }
                    };

                    for (_, name, entity) in players.iter_mut() {
                        if player.eq(name.as_str()) {
                            commands.entity(entity).despawn();
                        }
                    }
                },
                _ => {
                    warn!("Un message s'est perdu! {:?}", packet);
                }
            }
        }
    
        for ack in transport.recv.acks.drain() {
            debug!("{:?}", ack);
        }
    }
    
}