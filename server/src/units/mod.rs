use bevy::prelude::*;
use bevy_quinnet::server::QuinnetServer;
use common::{ArenaPos, Direction, ServerChannel, ServerMessage, Unit};

mod archer_tower;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(archer_tower::plugin);
}

pub(super) trait Spawn {
    fn spawn(&self, pos: ArenaPos, server: &mut QuinnetServer);
}

impl Spawn for Unit {
    fn spawn(&self, pos: ArenaPos, server: &mut QuinnetServer) {
        server
            .endpoint_mut()
            .broadcast_message_on(
                ServerChannel::UnorderedReliable,
                ServerMessage::SpawnUnit(*self, pos, Direction::Down),
            )
            .unwrap();
    }
}
