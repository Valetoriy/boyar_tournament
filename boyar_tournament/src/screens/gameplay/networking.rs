use bevy::prelude::*;
use bevy_quinnet::client::{
    certificate::CertificateVerificationMode, connection::ClientEndpointConfiguration,
    QuinnetClient, QuinnetClientPlugin,
};
use common::{ClientChannel, ServerMessage, LOCAL_BIND_IP, SERVER_HOST, SERVER_PORT};
use log::info;

use crate::screens::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(QuinnetClientPlugin::default());

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

fn handle_server_messages(mut client: ResMut<QuinnetClient>) {
    while let Some((_, message)) = client
        .connection_mut()
        .try_receive_message::<ServerMessage>()
    {
        match message {
            ServerMessage::StartGame => info!("Received StartGame"),
            ServerMessage::SpawnUnit(unit, arena_pos, direction) => {
                info!("Received SpawnUnit {unit:?}, {arena_pos:?}, {direction:?}")
            }
        }
    }
}
