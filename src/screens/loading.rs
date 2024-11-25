use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::scaling::ScaledTransform;

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(GameState::Loading).continue_to_state(GameState::Gameplay),
    );

    app.add_systems(OnEnter(GameState::Loading), spawn_loading_screen);
}

fn spawn_loading_screen(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn((
        SpriteBundle {
            texture: asset_server.load("screens/loading/loading.png"),
            ..default()
        },
        StateScoped(GameState::Loading),
        ScaledTransform::new(2., (0., 0.)),
    ));
    cmd.spawn(AudioBundle {
        source: asset_server.load("screens/loading/loading.ogg"),
        settings: PlaybackSettings::DESPAWN,
    });
}
