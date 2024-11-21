use bevy::{math::vec2, prelude::*, window::WindowResized};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DrawRegion>();
    app.register_type::<DrawRegion>();
    app.register_type::<ScaleSize>();

    app.add_systems(PreUpdate, update_draw_region);
    app.add_systems(Update, update_scaled_sprites);

    #[cfg(debug_assertions)]
    app.add_systems(Update, draw_draw_region_outline);
}

/// Регион 9x16(состоит из квадратов), внутри которого происходит вся отрисовка
/// Длина и ширина его сторон определяют размер для всех Sprite
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct DrawRegion {
    width: f32,
    height: f32,
}

/// Component для регулирования размеров Sprite
/// Значения равны доле квадрата из DrawRegion
#[derive(Component, Reflect)]
#[reflect(Component)]
struct ScaleSize(f32, f32);

fn update_draw_region(
    mut draw_region: ResMut<DrawRegion>,
    mut resize_events: EventReader<WindowResized>,
) {
    for r_e in resize_events.read() {
        let (aspect_ratio_width, aspect_ratio_height) = (9., 16.);
        let (window_width, window_height) = (r_e.width, r_e.height);

        // При длинном окне, DrawRegion по y на весь экран
        if window_height < window_width / aspect_ratio_width * aspect_ratio_height {
            draw_region.height = window_height;
            draw_region.width = draw_region.height / aspect_ratio_height * aspect_ratio_width;
        } else {
            // При высоком окне, DrawRegion по x на весь экран
            draw_region.width = window_width;
            draw_region.height = draw_region.width / aspect_ratio_width * aspect_ratio_height;
        }
    }
}

fn update_scaled_sprites(
    mut scaled_sprites: Query<(&mut Sprite, &ScaleSize)>,
    draw_region: Res<DrawRegion>,
) {
    for (mut sprite, scale_size) in &mut scaled_sprites {
        sprite.custom_size = vec2(
            draw_region.width / 9. * scale_size.0,
            draw_region.height / 16. * scale_size.1,
        )
        .into();
    }
}

#[cfg(debug_assertions)]
fn draw_draw_region_outline(mut gizmos: Gizmos, draw_region: Res<DrawRegion>) {
    gizmos.grid_2d(
        Vec2::ZERO,
        0.,
        UVec2::new(9, 16),
        vec2(draw_region.width / 9., draw_region.height / 16.),
        Color::srgb(1., 0., 0.),
    ).outer_edges();
}
