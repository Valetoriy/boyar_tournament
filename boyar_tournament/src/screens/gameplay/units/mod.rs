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

trait SpawnTag {
    fn spawn_tag(self_num: PlayerNumber, player_num: PlayerNumber) -> &'static str;
}

impl SpawnTag for PlayerNumber {
    fn spawn_tag(self_num: PlayerNumber, player_num: PlayerNumber) -> &'static str {
        use PlayerNumber::*;
        match (self_num, player_num) {
            (One, One) | (Two, Two) => "u",
            _ => "d",
        }
    }
}

trait SpawnPosition {
    fn spawn_pos(&self, pos: ArenaPos) -> ArenaPos;
}

impl SpawnPosition for PlayerNumber {
    fn spawn_pos(&self, pos: ArenaPos) -> ArenaPos {
        match self {
            PlayerNumber::One => pos,
            PlayerNumber::Two => ArenaPos(-pos.0, -pos.1),
        }
    }
}
