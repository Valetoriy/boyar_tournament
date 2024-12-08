use bevy::{log::LogPlugin, prelude::*, utils::HashMap};
use bevy_quinnet::{
    server::{
        certificate::CertificateRetrievalMode, ConnectionEvent, QuinnetServer,
        QuinnetServerPlugin, ServerEndpointConfiguration,
    },
    shared::ClientId,
};
use common::{ServerChannel, ServerMessage, LOCAL_BIND_IP, SERVER_HOST, SERVER_PORT};

fn main() {
    App::new()
        .init_resource::<Lobby>()
        .add_plugins((
            MinimalPlugins,
            LogPlugin::default(),
            QuinnetServerPlugin::default(),
        ))
        .add_systems(Startup, start_listening)
        .add_systems(Update, handle_connection_events)
        .run();
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
        info!("Client connected: {}", client.id);

        lobby.insert(client.id, lobby_len);

        if lobby.len() == 2 {
            server
                .endpoint_mut()
                .broadcast_message_on(ServerChannel::OrderedReliable, ServerMessage::StartGame)
                .unwrap();
        }
    }
}
