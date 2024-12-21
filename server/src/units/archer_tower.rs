use bevy::prelude::*;
use bevy_quinnet::server::QuinnetServer;
use common::{
    ArenaPos, Health, PlayerNumber, Projectile, ServerChannel, ServerMessage, Unit, UnitState,
};

use crate::ai::{Attack, AttackTargetType, AttackType};

use super::UnitType;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_archer_tower);
}

#[derive(Event)]
pub struct SpawnArcherTower(pub ArenaPos, pub PlayerNumber);

#[derive(Component)]
#[require(
    Health(|| Health::new(800)),
    UnitType(|| UnitType::Ground),
    UnitState,
    Attack(|| Attack::new(AttackType::Ranged(Projectile::Bullet),
        AttackTargetType::All, 0.7, 8.)),
)]
struct ArcherTower;

fn spawn_archer_tower(
    trigger: Trigger<SpawnArcherTower>,
    mut server: ResMut<QuinnetServer>,
    mut cmd: Commands,
) {
    let SpawnArcherTower(pos, player_num) = trigger.event();

    let entity = cmd.spawn((ArcherTower, *pos, *player_num)).id();

    server
        .endpoint_mut()
        .broadcast_message_on(
            ServerChannel::UnorderedReliable,
            ServerMessage::SpawnUnit(entity, Unit::ArcherTower, *pos, *player_num),
        )
        .unwrap();
}
