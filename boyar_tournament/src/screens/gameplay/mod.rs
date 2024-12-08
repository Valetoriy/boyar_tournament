use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::scaling::{DynamicScale, DynamicTransform};

use super::GameState;

mod arena;
mod networking;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(AsepriteUltraPlugin);

    app.configure_loading_state(
        LoadingStateConfig::new(GameState::Loading).load_collection::<CardsAssets>(),
    );

    app.add_plugins((arena::plugin, networking::plugin));

    app.add_systems(OnEnter(GameState::Gameplay), spawn_test);
}

#[derive(AssetCollection, Resource)]
struct CardsAssets {
    #[asset(path = "cards.aseprite")]
    cards: Handle<Aseprite>,
}

fn spawn_test(mut cmd: Commands, cards_assets: ResMut<CardsAssets>) {
    for (i, card) in [
        ("red", -2.05),
        ("blue", -0.22),
        ("green", 1.62),
        ("yellow", 3.45),
    ]
    .iter()
    .enumerate()
    {
        cmd.spawn((
            Name::new(format!("Карта {}", i + 1)),
            AseSpriteSlice {
                name: card.0.into(),
                aseprite: cards_assets.cards.clone(),
            },
            StateScoped(GameState::Gameplay),
            DynamicScale(1.8),
            DynamicTransform(card.1, -6.28),
        ));
    }
}
