use bevy::{prelude::*, utils::HashMap};
use bevy_quinnet::{
    server::{
        certificate::CertificateRetrievalMode, ConnectionEvent, QuinnetServer,
        QuinnetServerPlugin, ServerEndpointConfiguration,
    },
    shared::ClientId,
};
use common::{
    ArenaPos, ClientMessage, Direction, ServerChannel, ServerMessage, Unit, LOCAL_BIND_IP,
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
struct Lobby(HashMap<ClientId, u8>); // u8 - индекс игрока (0 или 1)

fn handle_connection_events(
    mut connection_events: EventReader<ConnectionEvent>,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<QuinnetServer>,
) {
    let lobby_len = lobby.len() as u8;
    for client in connection_events.read() {
        if lobby_len >= 2 {
            server.endpoint_mut().disconnect_client(client.id).unwrap();
            continue;
        }
        lobby.insert(client.id, lobby_len);

        if lobby.len() == 2 {
            server
                .endpoint_mut()
                .broadcast_message_on(ServerChannel::OrderedReliable, ServerMessage::StartGame)
                .unwrap();

            Unit::ArcherTower.spawn(ArenaPos(0., 3.5), &mut server);
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
