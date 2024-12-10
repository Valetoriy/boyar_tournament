use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::prelude::*;
use common::{ArenaPos, Health, PlayerNumber};

use crate::{scaling::DynamicScale, screens::GameState};

use super::{SpawnPosition, SpawnTag};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_archer_tower);

    app.configure_loading_state(
        LoadingStateConfig::new(GameState::Loading).load_collection::<ArcherTowerAssets>(),
    );
}

#[derive(Event)]
pub struct SpawnArcherTower(pub ArenaPos, pub PlayerNumber);

#[derive(Component)]
#[require(
    Health,

    Name(|| Name::new("Башня лучника")),
    DynamicScale(|| DynamicScale(0.55)),
    StateScoped<GameState>(|| StateScoped(GameState::Gameplay)),
)]
struct ArcherTower;

#[derive(Resource, AssetCollection)]
struct ArcherTowerAssets {
    #[asset(path = "units/musketeer/musketeer.aseprite")]
    sprite: Handle<Aseprite>,
}

fn spawn_archer_tower(
    trigger: Trigger<SpawnArcherTower>,
    mut cmd: Commands,
    self_num: Res<PlayerNumber>,
    assets: ResMut<ArcherTowerAssets>,
) {
    let SpawnArcherTower(pos, player_num) = trigger.event();

    let pos = self_num.spawn_pos(*pos);

    cmd.spawn((
        ArcherTower,
        pos,
        *player_num,
        AseSpriteAnimation {
            animation: Animation::tag(PlayerNumber::spawn_tag(*self_num, *player_num)),
            aseprite: assets.sprite.clone(),
        },
        // ManualTick,
    ));
}
