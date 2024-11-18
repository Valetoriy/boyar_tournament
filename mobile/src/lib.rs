use bevy::prelude::*;
use bevy::window::WindowMode;
use boyar_tournament::GamePlugin;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }
                .into(),
                ..default()
            }),
            GamePlugin,
        ))
        .run();
}
