use bevy::prelude::*;
use bevy_quinnet::server::QuinnetServer;
use common::{ArenaPos, Health, PlayerNumber, ServerChannel, ServerMessage, Unit};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_archer_tower);
}

#[derive(Event, Default)]
pub struct SpawnArcherTower(pub ArenaPos, pub PlayerNumber);

#[derive(Component)]
#[require(
    Health(|| Health::new(800))
)]
struct ArcherTower;

fn spawn_archer_tower(
    trigger: Trigger<SpawnArcherTower>,
    mut server: ResMut<QuinnetServer>,
    mut cmd: Commands,
) {
    let SpawnArcherTower(pos, player_num) = trigger.event();

    cmd.spawn((ArcherTower, *pos, *player_num));

    server
        .endpoint_mut()
        .broadcast_message_on(
            ServerChannel::UnorderedReliable,
            ServerMessage::SpawnUnit(Unit::ArcherTower, *pos, *player_num),
        )
        .unwrap();
}
