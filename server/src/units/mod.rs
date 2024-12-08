use bevy::prelude::*;

mod archer_tower;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(archer_tower::plugin);
}
