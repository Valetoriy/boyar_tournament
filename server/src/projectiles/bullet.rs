use bevy::prelude::*;
use bevy_quinnet::server::QuinnetServer;
use common::{ArenaPos, Health, Projectile, ServerChannel, ServerMessage};

use crate::ai::MovementSpeed;

use super::ProjectileRadius;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_bullet);

    app.add_systems(FixedUpdate, update_bullets);
}

#[derive(Event)]
pub struct SpawnBullet(pub Entity, pub Entity, pub ArenaPos);

#[derive(Component)]
#[require(
    Projectile(|| Projectile::Bullet),
    ProjectileRadius(|| ProjectileRadius(0.2)),
    MovementSpeed(|| MovementSpeed(5.)),
)]
struct Bullet(Entity, Entity);

fn spawn_bullet(
    trigger: Trigger<SpawnBullet>,
    mut server: ResMut<QuinnetServer>,
    mut cmd: Commands,
) {
    let SpawnBullet(attacker, receiver, pos) = trigger.event();

    let entity = cmd.spawn((Bullet(*attacker, *receiver), *pos)).id();

    server
        .endpoint_mut()
        .broadcast_message_on(
            ServerChannel::UnorderedReliable,
            ServerMessage::SpawnProjectile(
                entity,
                Projectile::Bullet,
                *attacker,
                *receiver,
                *pos,
            ),
        )
        .unwrap();
}

fn update_bullets(
    mut bullets: Query<(
        Entity,
        &Bullet,
        &ProjectileRadius,
        &MovementSpeed,
        &mut ArenaPos,
    )>,
    mut units: Query<(&ArenaPos, &mut Health)>,
    mut cmd: Commands,
    mut server: ResMut<QuinnetServer>,
    time: Res<Time>,
) {
    for (entity, bullet, radius, ms, mut pos) in &mut bullets {
        let Ok((recv_pos, mut recv_health)) = units.get_mut(bullet.1) else {
            cmd.entity(entity).despawn();
            server
                .endpoint_mut()
                .broadcast_message_on(
                    ServerChannel::UnorderedReliable,
                    ServerMessage::Despawn(entity),
                )
                .unwrap();
            continue;
        };

        let direction = (*recv_pos - *pos).normalize();
        *pos -= direction.mul(ms.0 * time.elapsed_secs());

        if ((pos.0 - recv_pos.0).powi(2) + (pos.1 - recv_pos.1).powi(2)).sqrt() >= radius.0 {
            continue;
        }

        recv_health.0 = recv_health.0.saturating_sub(50);
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
