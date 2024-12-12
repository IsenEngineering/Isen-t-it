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
    pub skin: u8,
    pub vitesse: f32, // vitesse par update
    pub angle: i16, // angle de direction
    pub last_tick: f32
}

impl Player {
    pub fn new(position: Vec3, skin: u8) -> Self {
        return Self {
            position,
            skin,
            vitesse: 0.0,
            angle: 0,
            last_tick: 0.0
        }
    }
    pub fn update(&mut self, position: Vec3, elapsed: f32) {
        let delta = position.xy() - self.position.xy();
        let delta_t = elapsed - self.last_tick;

        self.vitesse = delta.distance(Vec2::ZERO) / delta_t;
        self.angle = (
            // teta = atan(y / x) en radian
            // * 180 / pi pour passer en degree
            f32::atan2(delta.y, delta.x).to_degrees() % 255.0
        ) as i16;
        
        self.last_tick = elapsed;
        self.position = position;
    }
}

pub type ClientConnection = Player;
pub type ClientsConnection = HashMap<String, Player>;

pub type ClientToServer = Vec3;

pub type ServerToClient = HashMap<String, Vec3>;

pub type ClientDeconnection = String;