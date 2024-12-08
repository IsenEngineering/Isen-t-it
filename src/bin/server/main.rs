use bevy::{log::LogPlugin, prelude::*};

mod network;

fn main() {
    // Application Bevy
    let mut app = App::new();

    app.add_plugins((
        MinimalPlugins,
        LogPlugin::default(),
        network::Reseau,
    ));
    app.run();
}