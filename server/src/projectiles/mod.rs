use bevy::prelude::*;
use bullet::SpawnBullet;
use common::{ArenaPos, Projectile};

mod bullet;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(bullet::plugin);
}

#[derive(Component)]
struct ProjectileRadius(pub f32);

pub(super) trait SpawnProjectile {
    fn spawn(&self, attacker: Entity, receiver: Entity, pos: ArenaPos, cmd: &mut Commands);
}

impl SpawnProjectile for Projectile {
    fn spawn(&self, attacker: Entity, receiver: Entity, pos: ArenaPos, cmd: &mut Commands) {
        match self {
            Projectile::Bullet => cmd.trigger(SpawnBullet(attacker, receiver, pos)),
        }
    }
}
