use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::scaling::ScaledTransform;

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevySprityPlugin);

    app.configure_loading_state(
        LoadingStateConfig::new(GameState::Loading)
            .load_collection::<CardsAssets>()
            .load_collection::<ArenaAssets>(),
    );

    app.add_systems(OnEnter(GameState::Gameplay), spawn_test);
}

#[derive(AssetCollection, Resource)]
struct ArenaAssets {
    #[asset(path = "reference/cr_far.png")]
    arena_reference: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
struct CardsAssets {
    #[asset(path = "cards.aseprite")]
    cards: Handle<Aseprite>,
}

fn spawn_test(
    mut cmd: Commands,
    arena_assets: ResMut<ArenaAssets>,
    cards_assets: ResMut<CardsAssets>,
) {
    cmd.spawn((
        Name::new("Референс арены"),
        SpriteBundle {
            texture: arena_assets.arena_reference.clone(),
            ..default()
        },
        StateScoped(GameState::Gameplay),
        ScaledTransform::new(1., (0., 0.)),
    ))
    .insert(Transform::from_translation(Vec3::ZERO.with_z(-1.)));

    for (i, card) in [
        ("red", -2.5),
        ("blue", -0.67),
        ("green", 1.17),
        ("yellow", 3.),
    ]
    .iter()
    .enumerate()
    {
        cmd.spawn((
            Name::new(format!("Карта {}", i + 1)),
            AsepriteSliceBundle {
                slice: AsepriteSlice::new(card.0),
                aseprite: cards_assets.cards.clone(),
                ..default()
            },
            StateScoped(GameState::Gameplay),
            ScaledTransform::new(1.8, (card.1, -6.28)),
        ));
    }
}
