use aeronet::io::{connection::{DisconnectReason, Disconnected, LocalAddr}, server::Server, Session};
use aeronet_webtransport::server::{SessionRequest, SessionResponse};
use bevy::prelude::*;

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

pub fn on_connected(trigger: Trigger<OnAdd, Session>, clients: Query<&Parent>) {
    let client = trigger.entity();
    let Ok(server) = clients.get(client).map(Parent::get) else {
        return;
    };

    info!("{client} connecté à {server}");
}

pub fn on_disconnected(trigger: Trigger<Disconnected>, clients: Query<&Parent>) {
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
}