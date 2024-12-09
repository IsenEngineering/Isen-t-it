use bevy::{prelude::*, utils::HashMap};
use aeronet::transport::lane::LaneKind;
use serde::{Deserialize, Serialize};

pub const TRANSPORT_LANES: [LaneKind; 4] = [
    LaneKind::ReliableOrdered, // Client introduce their selve
    LaneKind::UnreliableUnordered, // Clients send their positions
    LaneKind::UnreliableUnordered, // Server send clients' positions
    LaneKind::ReliableOrdered // Server send clients' deconnections
];

#[derive(Component, Clone, Deserialize, Serialize, Debug)]
pub struct Player {
    pub position: Vec3,
    pub skin: u8
}

pub type ClientConnection = Player;
pub type ClientsConnection = HashMap<String, Player>;

pub type ClientToServer = Vec3;

pub type ServerToClient = HashMap<String, Vec3>;

pub type ClientDeconnection = String;