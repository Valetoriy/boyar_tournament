#[cfg(debug_assertions)]
mod dev_tools;
mod scaling;

use bevy::{audio::Volume, prelude::*};

#[derive(States, Debug, PartialEq, Eq, Clone, Hash, Default)]
enum GameState {
    #[default]
    // Splash,
    // Loading,
    // Menu,
    Gameplay,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.enable_state_scoped_entities::<GameState>();

        app.add_systems(Startup, spawn_camera);

        app.insert_resource(GlobalVolume {
            volume: Volume::new(0.3),
        });

        app.add_plugins(scaling::plugin);

        #[cfg(debug_assertions)]
        app.add_plugins(dev_tools::plugin);
    }
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn((Camera2dBundle::default(), IsDefaultUiCamera));
}
