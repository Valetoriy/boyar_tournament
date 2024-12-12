use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{scaling::DynamicScale, screens::GameState};

use crate::scaling::DrawRegion;
use common::ArenaPos;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ArenaPos>();
    app.register_type::<ArenaHeightOffset>();

    app.configure_loading_state(
        LoadingStateConfig::new(GameState::Loading).load_collection::<ArenaAssets>(),
    );

    app.add_systems(OnEnter(GameState::Gameplay), spawn_arena);

    app.add_systems(
        Update,
        update_arena_pos.run_if(in_state(GameState::Gameplay)),
    );

    #[cfg(debug_assertions)]
    {
        app.add_systems(
            Update,
            draw_arena_region_outline.run_if(in_state(GameState::Gameplay)),
        );
    }
}

#[derive(AssetCollection, Resource)]
struct ArenaAssets {
    #[asset(path = "arena/arena_template.aseprite")]
    arena: Handle<Aseprite>,
    // #[asset(path = "arena/battle.ogg")]
    // battle_music: Handle<AudioSource>,
}

fn spawn_arena(mut cmd: Commands, arena_assets: ResMut<ArenaAssets>) {
    cmd.spawn((
        Name::new("Шаблон арены"),
        AseSpriteSlice {
            name: "arena_template".into(),
            aseprite: arena_assets.arena.clone(),
        },
        StateScoped(GameState::Gameplay),
        DynamicScale(1.),
        Transform::from_translation(Vec3::ZERO.with_z(-0.5)),
    ));
    // cmd.spawn((
    //     AudioPlayer::new(arena_assets.battle_music.clone()),
    //     PlaybackSettings::LOOP,
    //     StateScoped(GameState::Gameplay),
    // ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ArenaHeightOffset(pub f32);

fn update_arena_pos(
    mut arena_pos: Query<(&mut Transform, &ArenaPos, Option<&ArenaHeightOffset>)>,
    draw_region: Res<DrawRegion>,
) {
    for (mut transform, arena_pos, height_offset) in &mut arena_pos {
        transform.translation.x = arena_pos.0 * draw_region.width / 19.61;
        transform.translation.y =
            arena_pos.1 * draw_region.height / 43.2 + draw_region.height / 13.5;

        // Чем ниже сущность на арене тем "выше" она отображается
        transform.translation.z = transform.translation.y / draw_region.height * -1.;

        if let Some(height_offset) = height_offset {
            transform.translation.y += height_offset.0 * draw_region.height / 43.2;
        }
    }
}

#[cfg(debug_assertions)]
fn draw_arena_region_outline(
    mut toggle: Local<bool>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut gizmos: Gizmos,
    draw_region: Res<DrawRegion>,
) {
    use bevy::math::vec2;

    if keyboard.just_pressed(KeyCode::F2) {
        *toggle ^= true;
    }
    if !*toggle {
        return;
    }

    gizmos
        .grid_2d(
            Isometry2d::from_translation(vec2(0., draw_region.height / 13.5)),
            UVec2::new(18, 32),
            vec2(draw_region.width / 19.61, draw_region.height / 43.2),
            Color::srgb(1., 0.65, 0.),
        )
        .outer_edges();
}
