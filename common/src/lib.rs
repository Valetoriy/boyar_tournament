use std::{
    net::Ipv4Addr,
    ops::{Sub, SubAssign},
};

use bevy::{math::vec2, prelude::*};
use bevy_quinnet::shared::channels::{ChannelId, ChannelType, ChannelsConfiguration};
use serde::{Deserialize, Serialize};

pub const SERVER_HOST: Ipv4Addr = Ipv4Addr::LOCALHOST;
pub const LOCAL_BIND_IP: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
pub const SERVER_PORT: u16 = 42069;

#[derive(
    Debug,
    Component,
    Reflect,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Default,
    PartialEq,
    PartialOrd,
)]
#[reflect(Component)]
pub struct ArenaPos(pub f32, pub f32);
impl Sub for ArenaPos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        ArenaPos(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl SubAssign for ArenaPos {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}
impl ArenaPos {
    pub fn normalize(&self) -> Self {
        let v = vec2(self.0, self.1).normalize();
        ArenaPos(v.x, v.y)
    }
    pub fn mul(&self, n: f32) -> Self {
        ArenaPos(self.0 * n, self.1 * n)
    }
}

#[derive(Debug, Component, Serialize, Deserialize, Clone, Copy, Reflect)]
#[reflect(Component)]
pub enum Card {
    Rus,
    Musketeer,
    ThreeMusketeers,
    Priest,
    Bats,
    BatHorde,
    Giant,
    Bomber,
}

#[derive(Debug, Component, Serialize, Deserialize, Clone, Copy)]
pub enum Unit {
    ArcherTower,
    KingTower,
}

#[derive(Debug, Component, Serialize, Deserialize, Clone, Copy)]
pub enum Projectile {
    Bullet,
}

#[derive(Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Health(pub u16, pub u16); // Текущее и максимальное здоровье
impl Health {
    // Конкретное значение указывается в сервере, default для спауна на клиенте
    pub fn new(amount: u16) -> Self {
        Health(amount, amount)
    }
}
impl Default for Health {
    fn default() -> Self {
        Self::new(100)
    }
}

#[derive(Component, Debug, Serialize, Deserialize, Clone, Copy, Reflect)]
#[reflect(Component)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone, Copy, Reflect, Default)]
#[reflect(Component)]
pub enum UnitState {
    #[default]
    Idle,
    Moving,
    Attacking,
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    PlayCard { card: Card, placement: ArenaPos },
}

#[derive(
    Resource,
    Component,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    Default,
    Hash,
    Eq,
    PartialEq,
)]
pub enum PlayerNumber {
    #[default]
    One, // Игрок "снизу"
    Two, // Игрок "сверху"
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    StartGame(PlayerNumber),
    SpawnUnit(Entity, Unit, ArenaPos, PlayerNumber),
    // 2 Entity - атакующий и цель
    SpawnProjectile(Entity, Projectile, Entity, Entity, ArenaPos),
    Despawn(Entity),
}

#[repr(u8)]
pub enum ClientChannel {
    // Разыгрывание карт, и мб вызов эмоутов
    OrderedReliable,
}
impl From<ClientChannel> for ChannelId {
    fn from(value: ClientChannel) -> Self {
        value as _
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
impl From<ServerChannel> for ChannelId {
    fn from(value: ServerChannel) -> Self {
        value as _
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
