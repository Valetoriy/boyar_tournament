use archer_tower::SpawnArcherTower;
use bevy::prelude::*;
use common::{ArenaPos, PlayerNumber, Unit};
use king_tower::SpawnKingTower;

mod archer_tower;
mod king_tower;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((archer_tower::plugin, king_tower::plugin));
}

#[derive(Component)]
pub enum UnitType {
    Air,
    Ground,
}

pub(super) trait SpawnUnit {
    fn spawn(&self, pos: ArenaPos, player_num: PlayerNumber, cmd: &mut Commands);
}

impl SpawnUnit for Unit {
    fn spawn(&self, pos: ArenaPos, player_num: PlayerNumber, cmd: &mut Commands) {
        match self {
            Unit::ArcherTower => cmd.trigger(SpawnArcherTower(pos, player_num)),
            Unit::KingTower => cmd.trigger(SpawnKingTower(pos, player_num)),
        }
    }
}
