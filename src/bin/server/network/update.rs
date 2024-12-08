use aeronet::transport::Transport;
use bevy::prelude::*;
use bincode;
use isent_it::network::OutgoingUpdate;

pub fn recv(
    mut transports: Query<&mut Transport>
) {
    for mut transport in transports.iter_mut() {
        for packet in transport.recv.msgs.drain() {
            info!("incomming packet: {:?}", packet);

            let update = match bincode::deserialize::<OutgoingUpdate>(
                packet.payload.as_slice()) {
                Ok(u) => u,
                Err(e) => {
                    warn!("couldn't deserialize OutgoingUpdate: {e}");
                    continue
                }
            };

            info!("deserialized packet : {:?}", update);
        }
    
        for _ in transport.recv.acks.drain() {}
    }
}