use archer_tower::SpawnArcherTower;
use bat::SpawnBat;
use bevy::prelude::*;
use common::{ArenaPos, PlayerNumber, Unit};
use king_tower::SpawnKingTower;
use musketeer::SpawnMusketeer;
use priest::SpawnPriest;
use rus::SpawnRus;

mod archer_tower;
mod bat;
mod king_tower;
mod musketeer;
mod priest;
mod rus;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        archer_tower::plugin,
        king_tower::plugin,
        rus::plugin,
        musketeer::plugin,
        bat::plugin,
        priest::plugin,
    ));
}

#[derive(Component)]
pub enum UnitType {
    Air,
    Ground,
}

#[derive(Component)]
pub struct Hitbox(pub f32);

pub(super) trait SpawnUnit {
    fn spawn(&self, pos: ArenaPos, player_num: PlayerNumber, cmd: &mut Commands);
}

impl SpawnUnit for Unit {
    fn spawn(&self, pos: ArenaPos, player_num: PlayerNumber, cmd: &mut Commands) {
        match self {
            Unit::ArcherTower => cmd.trigger(SpawnArcherTower(pos, player_num)),
            Unit::KingTower => cmd.trigger(SpawnKingTower(pos, player_num)),
            Unit::Rus => cmd.trigger(SpawnRus(pos, player_num)),
            Unit::Musketeer => cmd.trigger(SpawnMusketeer(pos, player_num)),
            Unit::Bat => cmd.trigger(SpawnBat(pos, player_num)),
            Unit::Priest => cmd.trigger(SpawnPriest(pos, player_num)),
        }
    }
}
