use aeronet::transport::lane::LaneIndex;
use aeronet::{io::Session, transport::Transport};
use bevy::{prelude::*, utils::Instant};
use isent_it::joueur::composants::JoueurPrincipal;
use isent_it::network::OutgoingUpdate;

pub fn send(
    mut transports: Query<&mut Transport, With<Session>>,
    positions: Query<&Transform, (With<JoueurPrincipal>, Changed<Transform>)>
) {
    let mut transport = match transports.get_single_mut() {
        Ok(t) => t,
        _ => return
    };
    let position = match positions.get_single() {
        Ok(p) => p,
        _ => return
    };

    let update = OutgoingUpdate {
        position: position.translation,
        skin: 0
    };

    info!("{:?}", update);

    transport.send.push(
        LaneIndex(0), 
        bincode::serialize(&update).unwrap().into(),
        Instant::now()
    );
}

pub fn recv(
    mut transports: Query<&mut Transport, With<Session>>
) {
    let mut transport = match transports.get_single_mut() {
        Ok(t) => t,
        _ => return
    };
    
    for packet in transport.recv.msgs.drain() {
        info!("incomming packet: {:?}", packet);
    }

    for _ in transport.recv.acks.drain() {}
}