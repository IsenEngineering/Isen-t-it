use aeronet::{io::{connection::{DisconnectReason, Disconnected, LocalAddr}, server::Server, Session}, transport::{lane::LaneIndex, Transport}};
use aeronet_webtransport::server::{SessionRequest, SessionResponse};
use bevy::{prelude::*, utils::{hashbrown::HashMap, Instant}};

pub fn on_opened(trigger: Trigger<OnAdd, Server>, servers: Query<&LocalAddr>) {
    let server = trigger.entity();
    let local_addr = servers.get(server)
        .expect("spawned session entity should have a name");
    info!("{server} ouvert sur {}", **local_addr);
}

pub fn on_session_request(
    mut trigger: Trigger<SessionRequest>,
    clients: Query<&Parent>,
) {
    let client = trigger.entity();
    let request = trigger.event_mut();
    let Ok(server) = clients.get(client).map(Parent::get) else {
        return;
    };

    info!("{client} connecting to {server} with headers:");
    for (header_key, header_value) in &request.headers {
        info!("  {header_key}: {header_value}");
    }

    request.respond(SessionResponse::Accepted);
}

use isent_it::network::{ClientsConnection, Player, TRANSPORT_LANES};
pub fn on_connected(
    trigger: Trigger<OnAdd, Session>, 
    mut commands: Commands,
    sessions: Query<&Session>,
    players: Query<(&Player, Entity)>) {
    let client = trigger.entity();

    let session = sessions.get(client)
        .expect("Should be connected");

    let mut transport = Transport::new(
        session, 
        TRANSPORT_LANES, 
        TRANSPORT_LANES, 
        Instant::now()
    ).unwrap();

    let mut connected: ClientsConnection = HashMap::new();
    for (player, entity) in players.iter() {
        let name = entity.to_bits().to_string();
        connected.insert(
            name, 
            player.clone()
        );
    }

    transport.send.push(
        LaneIndex(0), 
        bincode::serialize(&connected).unwrap().into(),
        Instant::now()
    );

    commands.entity(client).insert(transport);

    info!("{client} connecté");
}

pub fn on_disconnected(
    trigger: Trigger<Disconnected>, 
    clients: Query<&Parent>,
    mut transports: Query<&mut Transport>) {
    let client = trigger.entity();
    let Ok(server) = clients.get(client).map(Parent::get) else {
        return;
    };

    match &trigger.event().reason {
        DisconnectReason::User(reason) => {
            info!("{client} disconnected from {server} by user: {reason}");
        }
        DisconnectReason::Peer(reason) => {
            info!("{client} disconnected from {server} by peer: {reason}");
        }
        DisconnectReason::Error(err) => {
            warn!("{client} disconnected from {server} due to error: {err:#}");
        }
    }

    let id = client.to_bits().to_string();
    for mut transport in transports.iter_mut() {
        transport.send.push(
            LaneIndex(3), 
            bincode::serialize(&id).unwrap().into(), 
            Instant::now()
        );
    }
}