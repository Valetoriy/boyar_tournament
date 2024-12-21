use bevy::prelude::*;
use bevy_quinnet::server::QuinnetServer;
use common::{ArenaPos, Health, Projectile, ServerChannel, ServerMessage, UnitState};

use crate::projectiles::SpawnProjectile;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (update_attacks, update_unit_state, check_health),
    );
}

pub enum AttackType {
    Melee(u16), // Урон
    Ranged(Projectile),
}

pub enum AttackTargetType {
    Ground,
    All,
}

#[derive(Component)]
pub struct Attack {
    target: Option<Entity>,
    a_type: AttackType,
    t_type: AttackTargetType,
    cooldown_timer: Timer,
    range: f32,
}
impl Attack {
    pub fn new(a_type: AttackType, targets: AttackTargetType, cd: f32, range: f32) -> Self {
        Self {
            target: None,
            a_type,
            t_type: targets,
            cooldown_timer: Timer::from_seconds(cd, TimerMode::Repeating),
            range,
        }
    }
}

fn update_attacks(
    mut attacks: Query<(Entity, &mut Attack)>,
    mut units: Query<(&ArenaPos, &mut Health)>,
    time: Res<Time>,
    mut cmd: Commands,
) {
    for (attacker, mut attack) in &mut attacks {
        // target есть только в UnitState::Attacking
        let Some(receiver) = attack.target else {
            attack.cooldown_timer.reset();
            continue;
        };
        let Ok((_, mut health)) = units.get_mut(receiver) else {
            // Все мертвы
            attack.target = None;
            continue;
        };

        if !attack.cooldown_timer.tick(time.elapsed()).just_finished() {
            continue;
        }

        match attack.a_type {
            AttackType::Melee(damage) => health.0 = health.0.saturating_sub(damage),
            AttackType::Ranged(projectile) => {
                let (pos, _) = units.get(attacker).unwrap();
                projectile.spawn(attacker, receiver, *pos, &mut cmd)
            }
        }
    }
}

fn check_health(
    query: Query<(Entity, &Health)>,
    mut server: ResMut<QuinnetServer>,
    mut cmd: Commands,
) {
    for (entity, health) in &query {
        if health.0 == 0 {
            cmd.entity(entity).despawn();
            server
                .endpoint_mut()
                .broadcast_message_on(
                    ServerChannel::UnorderedReliable,
                    ServerMessage::Despawn(entity),
                )
                .unwrap();
        }
    }
}

#[derive(Component)]
pub struct MovementSpeed(pub f32);

#[derive(Component)]
pub struct StunnedTimer(pub Timer);

#[derive(Component)]
pub struct AggroRadius(pub f32);

fn update_unit_state(
    mut units: Query<
        (
            Entity,
            &mut UnitState,
            &Attack,
            Option<&AggroRadius>,
            Option<&MovementSpeed>,
        ),
        Without<StunnedTimer>,
    >,
    positions: Query<&ArenaPos, Without<Projectile>>,
) {
    for (entity, mut state, attack, aggro_radius, ms) in &mut units {
        match *state {
            UnitState::Idle | UnitState::Moving => todo!(),
            UnitState::Attacking => {
                if attack.target.is_none() {
                    *state = UnitState::Idle;
                }
            }
        }
    }
}
