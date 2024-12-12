use aeronet::{io::connection::{DisconnectReason, Disconnected}, transport::{AeronetTransportPlugin, TransportSet}};
use aeronet_websocket::client::{WebSocketClient, WebSocketClientPlugin};
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
            WebSocketClientPlugin,
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
            .add_observer(observers::on_disconnected)
            .add_observer(websocket_fallback);
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
    let config = match config::transport::config(CERT_HASH.to_string()) {
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


// Lorsqu'on détecte une déconnexion due à une erreur 
// (il y a de très très grandes chances que ça vient 
// de l'indisponibilité de l'API WebTransport)

// On switch sur du WebSocket (très largement supporté)
#[cfg(not(target_family = "wasm"))]
const DEFAULT_TARGET_SOCKET: &str = "wss://isent_it.aruni.space:25566";
#[cfg(target_family = "wasm")]
const DEFAULT_TARGET_SOCKET: &str = "ws://isent_it.aruni.space:25566";
fn websocket_fallback(
    trigger: Trigger<Disconnected>,
    mut commands: Commands,
    webtransports: Query<Entity, With<WebTransportClient>>
) {
    let Disconnected { reason } = trigger.event();
    match reason {
        DisconnectReason::Error(_) => {
            if webtransports.is_empty() {
                return
            }
            
            info!("Switching on Websocket connections");
            
            for webtransport in webtransports.iter() {
                commands.entity(webtransport).try_despawn();
            }
            let target = match var("TARGET_URL_SOCKET") {
                Ok(t) => t,
                _ => DEFAULT_TARGET_SOCKET.to_string()
            };

            let config = config::socket::config();

            let name = format!("{}. {target}", random::<u8>());
            commands
                .spawn(Name::new(name))
                .queue(WebSocketClient::connect(config, target));
        },
        _ => return
    }
}