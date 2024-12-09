use aeronet::transport::AeronetTransportPlugin;
use bevy::prelude::*;
use aeronet_webtransport::{
    cert::hash_to_b64, server::{WebTransportServer, WebTransportServerPlugin}, wtransport::Identity
};
use std::fs::write;

mod config;
mod observers;
mod update;

pub struct Reseau;

impl Plugin for Reseau {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WebTransportServerPlugin,
            AeronetTransportPlugin
        ));
        app.add_systems(PreStartup, listen);

        app.add_systems(PreUpdate, update::recv);
        app.add_systems(PostUpdate, (
            update::send_changes,
            update::send_connections
        ));

        app
            .add_observer(observers::on_connected)
            .add_observer(observers::on_disconnected)
            .add_observer(observers::on_opened)
            .add_observer(observers::on_session_request);
    }
}

fn listen(mut commands: Commands) {
    let identity = Identity::self_signed(
        [
            "localhost", 
            "127.0.0.1", 
            "::1"
        ]
    ).expect("all given SANs should be valid DNS names");

    let cert = &identity.certificate_chain().as_slice()[0];
    let cert_hash = hash_to_b64(cert.hash());
    info!("Certificate hash : {cert_hash}");

    write(".keys/hash", cert_hash).unwrap();

    let config = config::server_config(&identity);
    commands.spawn_empty().queue(WebTransportServer::open(config)); 
}