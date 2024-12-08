use aeronet_webtransport::server::WebTransportServerPlugin;
use bevy::{log::LogPlugin, prelude::*};

mod network;

fn main() {
    // Application Bevy
    let mut app = App::new();

    app.add_plugins((
        MinimalPlugins,
        LogPlugin::default(),
        WebTransportServerPlugin,
        network::Reseau,
    ));
    app.run();
}