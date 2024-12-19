use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::prelude::*;
use common::{ArenaPos, Health, PlayerNumber, UnitState};

use crate::{
    scaling::DynamicScale,
    screens::{gameplay::arena::ArenaHeightOffset, GameState},
};

use super::{SpawnPosition, SpawnTag};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_archer_tower);

    app.configure_loading_state(
        LoadingStateConfig::new(GameState::Loading).load_collection::<ArcherTowerAssets>(),
    );

    app.add_systems(OnExit(GameState::Gameplay), despawn_archer_towers);
}

#[derive(Event)]
pub struct SpawnArcherTower(pub ArenaPos, pub PlayerNumber);

#[derive(Component)]
#[require(
    Name(|| Name::new("Башня лучника")),
    DynamicScale(|| DynamicScale(0.55)),
    ArenaHeightOffset(|| ArenaHeightOffset(1.)),
)]
struct ArcherTower;

#[derive(Component)]
#[require(
    Health,

    Name(|| Name::new("Лучник на башне")),
    DynamicScale(|| DynamicScale(0.55)),
    ArenaHeightOffset(|| ArenaHeightOffset(3.1)),
)]
struct ArcherTowerArcher;

/// Требуется для привязки лучника к башне
#[derive(Component)]
struct AssociatedTower(Entity);

#[derive(Resource, AssetCollection)]
struct ArcherTowerAssets {
    #[asset(path = "units/archer_tower/ally_tower.aseprite")]
    ally_tower: Handle<Aseprite>,
    #[asset(path = "units/archer_tower/enemy_tower.aseprite")]
    enemy_tower: Handle<Aseprite>,

    #[asset(path = "units/musketeer/ally_musketeer.aseprite")]
    ally_archer: Handle<Aseprite>,
    #[asset(path = "units/musketeer/enemy_musketeer.aseprite")]
    enemy_archer: Handle<Aseprite>,
}

fn spawn_archer_tower(
    trigger: Trigger<SpawnArcherTower>,
    mut cmd: Commands,
    self_num: Res<PlayerNumber>,
    assets: ResMut<ArcherTowerAssets>,
) {
    let SpawnArcherTower(pos, player_num) = trigger.event();

    let mut pos = self_num.spawn_pos(*pos);

    let (tower_sprite, archer_sprite) = if pos.1 < 0. {
        (assets.ally_tower.clone(), assets.ally_archer.clone())
    } else {
        (assets.enemy_tower.clone(), assets.enemy_archer.clone())
    };

    let tower = cmd
        .spawn((
            ArcherTower,
            pos,
            AseSpriteSlice {
                name: "tower".into(),
                aseprite: tower_sprite,
            },
        ))
        .id();

    let tag = self_num.spawn_tag(*player_num);

    pos.1 -= 0.01;
    cmd.spawn((
        ArcherTowerArcher,
        pos,
        UnitState::Idle,
        AseSpriteAnimation {
            animation: Animation::tag(tag),
            aseprite: archer_sprite,
        },
        AssociatedTower(tower),
    ));
}

fn despawn_archer_towers(mut cmd: Commands, towers: Query<(Entity, &AssociatedTower)>) {
    for (archer, tower) in towers.iter() {
        cmd.entity(tower.0).despawn();
        cmd.entity(archer).despawn();
    }
}
