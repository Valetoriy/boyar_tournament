use archer_tower::SpawnArcherTower;
use bevy::prelude::*;
use common::{ArenaPos, PlayerNumber, Unit};

mod archer_tower;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(archer_tower::plugin);
}

pub(super) trait Spawn {
    fn spawn(&self, pos: ArenaPos, player_num: PlayerNumber, cmd: &mut Commands);
}

impl Spawn for Unit {
    fn spawn(&self, pos: ArenaPos, player_num: PlayerNumber, cmd: &mut Commands) {
        match self {
            Unit::ArcherTower => cmd.trigger(SpawnArcherTower(pos, player_num)),
        }
    }
}
