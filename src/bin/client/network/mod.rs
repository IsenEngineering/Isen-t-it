use aeronet_webtransport::client::{WebTransportClient, WebTransportClientPlugin};
use bevy::prelude::*;
use rand::random;

mod config;
mod observers;

pub struct Client;

impl Plugin for Client {
    fn build(&self, app: &mut App) {
        app.add_plugins(WebTransportClientPlugin);

        app.add_systems(PreStartup, connect);

        app
            .add_observer(observers::on_connected)
            .add_observer(observers::on_connecting)
            .add_observer(observers::on_disconnected);
    }
}

const TARGET: &str = "https://[::1]:25565";
const CERT_HASH: &str = include_str!("../../../../.keys/hash");

fn connect(mut commands: Commands) {
    let config = match config::client_config(CERT_HASH.to_string()) {
        Ok(config) => config,
        Err(err) => {
            warn!("Failed to create client config: {err:#}");
            return;
        }
    };

    let name = format!("{}. {TARGET}", random::<u8>());
    commands
        .spawn(Name::new(name))
        .queue(WebTransportClient::connect(config, TARGET));
}