use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};

mod network;
const TICK_RATE: f64 = 30.0;

fn main() {
    // Application Bevy
    let mut app = App::new();

    app.add_plugins((
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / TICK_RATE,
        ))),
        LogPlugin::default(),
        network::Reseau,
        TransformPlugin
    ));
    app.run();
}