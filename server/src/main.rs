use bevy::{log::LogPlugin, prelude::*};

mod networking;
mod units;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            LogPlugin::default(),
            units::plugin,
            networking::plugin,
        ))
        .run();
}
