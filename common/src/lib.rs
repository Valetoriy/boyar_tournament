use std::net::Ipv4Addr;

use bevy::prelude::*;
use bevy_quinnet::shared::channels::{ChannelId, ChannelType, ChannelsConfiguration};
use serde::{Deserialize, Serialize};

pub const SERVER_HOST: Ipv4Addr = Ipv4Addr::LOCALHOST;
pub const LOCAL_BIND_IP: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
pub const SERVER_PORT: u16 = 42069;

#[derive(Debug, Component, Reflect, Serialize, Deserialize, Clone, Copy)]
#[reflect(Component)]
pub struct ArenaPos(pub f32, pub f32);

#[derive(Debug, Component, Reflect, Serialize, Deserialize, Clone, Copy)]
#[reflect(Component)]
pub enum Unit {
    ArcherTower,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    PlayCard {
        card_number: u8, // Номер карты в текущей руке
        placement: ArenaPos,
    },
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    StartGame,
    SpawnUnit(Unit, ArenaPos, Direction),
}

#[repr(u8)]
pub enum ClientChannel {
    // Разыгрывание карт, и мб вызов эмоутов
    OrderedReliable,
}
impl Into<ChannelId> for ClientChannel {
    fn into(self) -> ChannelId {
        self as ChannelId
    }
}
impl ClientChannel {
    pub fn channels_config() -> ChannelsConfiguration {
        ChannelsConfiguration::from_types(vec![ChannelType::OrderedReliable]).unwrap()
    }
}

#[repr(u8)]
pub enum ServerChannel {
    // Инициализация
    OrderedReliable,
    // Рассылка действий игроков
    UnorderedReliable,
    // Синхронизация юнитов
    Unreliable,
}
impl Into<ChannelId> for ServerChannel {
    fn into(self) -> ChannelId {
        self as ChannelId
    }
}
impl ServerChannel {
    pub fn channels_config() -> ChannelsConfiguration {
        ChannelsConfiguration::from_types(vec![
            ChannelType::OrderedReliable,
            ChannelType::UnorderedReliable,
            ChannelType::Unreliable,
        ])
        .unwrap()
    }
}
