use bevy::prelude::*;
use bevy_quinnet::server::QuinnetServer;
use common::{
    ArenaPos, Health, PlayerNumber, Projectile, ServerChannel, ServerMessage, Unit, UnitState,
};

use crate::ai::{Attack, AttackTargetType, AttackType};

use super::UnitType;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_king_tower);
}

#[derive(Event)]
pub struct SpawnKingTower(pub ArenaPos, pub PlayerNumber);

#[derive(Component)]
#[require(
    Health(|| Health::new(1600)),
    UnitType(|| UnitType::Ground),
    UnitState,
    Attack(|| Attack::new(AttackType::Ranged(Projectile::Bullet),
        AttackTargetType::All, 0.7, 6.)),
)]
struct KingTower;

fn spawn_king_tower(
    trigger: Trigger<SpawnKingTower>,
    mut server: ResMut<QuinnetServer>,
    mut cmd: Commands,
) {
    let SpawnKingTower(pos, player_num) = trigger.event();

    let entity = cmd.spawn((KingTower, *pos, *player_num)).id();

    server
        .endpoint_mut()
        .broadcast_message_on(
            ServerChannel::UnorderedReliable,
            ServerMessage::SpawnUnit(entity, Unit::KingTower, *pos, *player_num),
        )
        .unwrap();
}
