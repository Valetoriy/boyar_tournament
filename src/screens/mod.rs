use bevy::prelude::*;

mod loading;
mod splash;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<GameState>();
    app.enable_state_scoped_entities::<GameState>();

    app.add_plugins((splash::plugin, loading::plugin));
}

#[derive(States, Debug, PartialEq, Eq, Clone, Hash, Default)]
pub enum GameState {
    #[default]
    Splash,
    Loading,
    // Menu,
    Gameplay,
}
