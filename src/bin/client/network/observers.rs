use aeronet::io::{connection::{DisconnectReason, Disconnected}, Session, SessionEndpoint};
use bevy::prelude::*;

pub fn on_connecting(
    trigger: Trigger<OnAdd, SessionEndpoint>,
    names: Query<&Name>,
) {
    let entity = trigger.entity();
    let name = names
        .get(entity)
        .expect("our session entity should have a name");

    info!("{name} connecting");
}

pub fn on_connected(
    trigger: Trigger<OnAdd, Session>,
    names: Query<&Name>,
) {
    let entity = trigger.entity();
    let name = names
        .get(entity)
        .expect("our session entity should have a name");
    info!("{name} connected");
}

pub fn on_disconnected(
    trigger: Trigger<Disconnected>,
    names: Query<&Name>,
) {
    let entity = trigger.entity();
    let Disconnected { reason } = trigger.event();
    let name = names
        .get(entity)
        .expect("our session entity should have a name");
    match reason {
        DisconnectReason::User(reason) => {
            info!("{name} disconnected by user: {reason}");
        }
        DisconnectReason::Peer(reason) => {
            info!("{name} disconnected by peer: {reason}");
        }
        DisconnectReason::Error(err) => {
            error!("{name} disconnected due to error: {err:#}");
        }
    };
}