use bevy::{prelude::*, utils::HashMap};
use aeronet::transport::lane::LaneKind;
use serde::{Deserialize, Serialize};

pub const TRANSPORT_LANES: [LaneKind; 2] = [
    LaneKind::ReliableOrdered,
    LaneKind::ReliableOrdered
];

#[derive(Serialize, Deserialize, Debug)]
pub struct OutgoingUpdate {
    pub position: Vec3,
    pub skin: u8
}

#[derive(Serialize, Deserialize)]
pub struct IncommingUpdate(pub HashMap<String, OutgoingUpdate>);