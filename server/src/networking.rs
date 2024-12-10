use bevy::{prelude::*, utils::HashMap};
use bevy_quinnet::{
    server::{
        certificate::CertificateRetrievalMode, ConnectionEvent, QuinnetServer,
        QuinnetServerPlugin, ServerEndpointConfiguration,
    },
    shared::ClientId,
};
use common::{
    ArenaPos, ClientMessage, PlayerNumber, ServerChannel, ServerMessage, Unit, LOCAL_BIND_IP,
    SERVER_HOST, SERVER_PORT,
};

use crate::units::Spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(QuinnetServerPlugin::default());

    app.init_resource::<Lobby>();
    app.add_systems(Startup, start_listening);
    app.add_systems(Update, (handle_connection_events, handle_client_messages));
}

fn start_listening(mut server: ResMut<QuinnetServer>) {
    server
        .start_endpoint(
            ServerEndpointConfiguration::from_ip(LOCAL_BIND_IP, SERVER_PORT),
            CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: SERVER_HOST.to_string(),
            },
            ServerChannel::channels_config(),
        )
        .unwrap();
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Lobby(HashMap<ClientId, PlayerNumber>);

fn handle_connection_events(
    mut connection_events: EventReader<ConnectionEvent>,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<QuinnetServer>,
    mut cmd: Commands,
) {
    let lobby_len = lobby.len() as u8;
    for client in connection_events.read() {
        if lobby_len >= 2 {
            server.endpoint_mut().disconnect_client(client.id).unwrap();
            continue;
        }
        use PlayerNumber::*;

        let player_num = match lobby_len {
            0 => One,
            1 => Two,
            _ => unreachable!(),
        };
        lobby.insert(client.id, player_num);

        if lobby.len() == 2 {
            // Отправить каждому игроку его PlayerNumber
            for (client_id, player_num) in lobby.iter() {
                server
                    .endpoint_mut()
                    .send_message_on(
                        *client_id,
                        ServerChannel::OrderedReliable,
                        ServerMessage::StartGame(*player_num),
                    )
                    .unwrap();
            }

            Unit::ArcherTower.spawn(ArenaPos(0., -3.5), One, &mut cmd);
            Unit::ArcherTower.spawn(ArenaPos(5.5, 3.5), Two, &mut cmd);
            Unit::ArcherTower.spawn(ArenaPos(-5.5, 3.5), Two, &mut cmd);
        }
    }
}

fn handle_client_messages(mut server: ResMut<QuinnetServer>) {
    let endpoint = server.endpoint_mut();
    for client_id in endpoint.clients() {
        while let Some((_, message)) =
            endpoint.try_receive_message_from::<ClientMessage>(client_id)
        {
            match message {
                ClientMessage::PlayCard {
                    card_number,
                    placement,
                } => info!("Received PlayCard №{card_number}@{placement:?}"),
            }
        }
    }
}
