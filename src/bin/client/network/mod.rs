use aeronet::transport::{AeronetTransportPlugin, TransportSet};
use aeronet_webtransport::client::{WebTransportClient, WebTransportClientPlugin};
use bevy::prelude::*;
use rand::random;
use update::ClientNetworkSet;
use std::env::var;

mod config;
mod observers;
mod update;

pub struct Client;

impl Plugin for Client {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WebTransportClientPlugin,
            AeronetTransportPlugin
        ));

        app.configure_sets(
                PreUpdate, 
                (
                    TransportSet::Poll,
                    ClientNetworkSet::Recv,
                ).chain()
            )
            .configure_sets(
                PostUpdate, 
                (
                    ClientNetworkSet::Send,
                    TransportSet::Flush,
                ).chain()
            );

        app.add_systems(PreStartup, connect);
        app.add_systems(
            PreUpdate, 
            update::recv.in_set(ClientNetworkSet::Recv)
        );
        app.add_systems(
            PostUpdate, 
            update::send.in_set(ClientNetworkSet::Send)
        );

        app
            .add_observer(observers::on_connected)
            .add_observer(observers::on_connecting)
            .add_observer(observers::on_disconnected);
    }
}

// `TARGET_URL="https://[::1]:25565" cargo ...` to run on localhost
const DEFAULT_TARGET: &str = "https://isent_it.aruni.space:25565";
const CERT_HASH: &str = include_str!("../../../../.keys/hash");

fn connect(mut commands: Commands) {
    let target = match var("TARGET_URL") {
        Ok(t) => t,
        _ => DEFAULT_TARGET.to_string()
    };
    let config = match config::client_config(CERT_HASH.to_string()) {
        Ok(config) => config,
        Err(err) => {
            warn!("Failed to create client config: {err:#}");
            return;
        }
    };

    let name = format!("{}. {target}", random::<u8>());
    commands
        .spawn(Name::new(name))
        .queue(WebTransportClient::connect(config, target));
}