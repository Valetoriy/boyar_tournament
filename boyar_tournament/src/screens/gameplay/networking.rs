use bevy::{prelude::*, utils::HashMap};
use bevy_quinnet::client::{
    certificate::CertificateVerificationMode, connection::ClientEndpointConfiguration,
    QuinnetClient, QuinnetClientPlugin,
};
use common::{
    ClientChannel, PlayerNumber, ServerMessage, LOCAL_BIND_IP, SERVER_HOST, SERVER_PORT,
};

use crate::screens::GameState;

use super::units::Spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(QuinnetClientPlugin::default());

    app.init_resource::<PlayerNumber>();
    app.init_resource::<NetworkMapping>();
    app.register_type::<NetworkMapping>();

    app.add_systems(OnEnter(GameState::Gameplay), start_connection);
    app.add_systems(
        Update,
        handle_server_messages.run_if(in_state(GameState::Gameplay)),
    );
}

fn start_connection(mut client: ResMut<QuinnetClient>) {
    client
        .open_connection(
            ClientEndpointConfiguration::from_ips(SERVER_HOST, SERVER_PORT, LOCAL_BIND_IP, 0),
            CertificateVerificationMode::SkipVerification,
            ClientChannel::channels_config(),
        )
        .unwrap();
}

fn handle_server_messages(
    mut client: ResMut<QuinnetClient>,
    mut player_num: ResMut<PlayerNumber>,
    mut cmd: Commands,
) {
    while let Some((_, message)) = client
        .connection_mut()
        .try_receive_message::<ServerMessage>()
    {
        match message {
            ServerMessage::StartGame(n) => *player_num = n,
            ServerMessage::SpawnUnit(entity, unit, pos, player_num) => {
                unit.spawn(entity, pos, player_num, &mut cmd);
            }
            ServerMessage::SpawnProjectile(
                entity,
                projectile,
                entity1,
                entity2,
                arena_pos,
            ) => todo!(),
            ServerMessage::Despawn(entity) => todo!(),
        }
    }
}

#[derive(Resource, Reflect, Default, Deref, DerefMut)]
#[reflect(Resource)]
// Сопоставление Entity сервера и клиента
pub struct NetworkMapping(HashMap<Entity, Entity>);
