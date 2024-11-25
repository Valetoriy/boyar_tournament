use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::scaling::ScaledTransform;

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevySprityPlugin);

    app.configure_loading_state(
        LoadingStateConfig::new(GameState::Loading)
            .load_collection::<FontAssets>()
            .load_collection::<RedAssets>(),
    );

    app.add_systems(OnEnter(GameState::Gameplay), spawn_test);
}

#[derive(AssetCollection, Resource)]
struct FontAssets {
    #[asset(path = "Keleti-Regular.ttf")]
    main_font: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
struct RedAssets {
    #[asset(path = "cards/red/red.aseprite")]
    model: Handle<Aseprite>,
    #[asset(path = "tixon.aseprite")]
    tixon: Handle<Aseprite>,
}

fn spawn_test(
    mut cmd: Commands,
    font_assets: ResMut<FontAssets>,
    red_assets: ResMut<RedAssets>,
) {
    cmd.insert_resource(ClearColor(Color::linear_rgb(0.569, 0.065, 0.073)));

    cmd.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Здесь будет калс...",
                TextStyle {
                    font: font_assets.main_font.clone(),
                    font_size: 70.,
                    color: Color::srgb(1., 1., 0.),
                },
            ),
            ..default()
        },
        StateScoped(GameState::Gameplay),
        ScaledTransform::new(1., (-0.5, 7.)),
    ));

    cmd.spawn((
        AsepriteAnimationBundle {
            aseprite: red_assets.model.clone(),
            animation: Animation::default().with_tag("la").with_speed(0.8),
            ..default()
        },
        StateScoped(GameState::Gameplay),
        ScaledTransform::new(5.5, (1.4, 3.)),
    ));
    cmd.spawn((
        AsepriteAnimationBundle {
            aseprite: red_assets.model.clone(),
            animation: Animation::default().with_tag("ra"),
            ..default()
        },
        StateScoped(GameState::Gameplay),
        ScaledTransform::new(5.5, (-1.2, 3.)),
    ));

    cmd.spawn((
        Text2dBundle {
            text: Text::from_section(
                "LONDON",
                TextStyle {
                    font: font_assets.main_font.clone(),
                    font_size: 70.,
                    color: Color::srgb(1., 1., 0.),
                },
            ),
            ..default()
        },
        StateScoped(GameState::Gameplay),
        ScaledTransform::new(1., (0., 0.)),
    ));

    cmd.spawn((
        AsepriteAnimationBundle {
            aseprite: red_assets.tixon.clone(),
            animation: Animation::default(),
            ..default()
        },
        StateScoped(GameState::Gameplay),
        ScaledTransform::new(7.5, (0., -4.)),
    ));
}
