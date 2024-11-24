use bevy::{math::vec2, prelude::*, window::WindowResized};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DrawRegion>();
    app.register_type::<DrawRegion>();
    app.register_type::<ScaledTransform>();

    // TODO: для дебаг сборок использовать версию с EventReader вместо Trigger чтобы не нужно
    // было дёргать окно для просмотра изменений ScaledTransform
    app.add_systems(PreUpdate, update_draw_region)
        .observe(update_scaled_transform);

    #[cfg(debug_assertions)]
    app.add_systems(Update, draw_draw_region_outline);
}

/// Регион 9x16(состоит из квадратов), внутри которого происходит вся отрисовка
/// Длина и ширина его сторон определяют размер для всех сущностей
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct DrawRegion {
    width: f32,
    height: f32,
}

#[derive(Event)]
struct UpdateScaling;

fn update_draw_region(
    mut cmd: Commands,
    mut draw_region: ResMut<DrawRegion>,
    mut resize_events: EventReader<WindowResized>,
) {
    if resize_events.is_empty() {
        return;
    }

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

    cmd.trigger(UpdateScaling);
}

/// Компонент для регулирования размеров Sprite
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScaledTransform {
    /// Значение scale в компоненте Transform при размере окна игры 1920x1080
    pub scale: f32,
    /// Расположение сущности в квадратах DrawRegion
    pub translation: (f32, f32),
}

impl ScaledTransform {
    pub fn new(scale: f32, translation: (f32, f32)) -> Self {
        Self { scale, translation }
    }
}

fn update_scaled_transform(
    _: Trigger<UpdateScaling>,
    mut scaled_transform: Query<(&mut Transform, &ScaledTransform)>,
    draw_region: Res<DrawRegion>,
) {
    for (mut transform, scaled_transform) in &mut scaled_transform {
        transform.scale = Vec3::splat(scaled_transform.scale) * draw_region.height / 1080.;

        transform.translation.x = scaled_transform.translation.0 * draw_region.width / 9.;
        transform.translation.y = scaled_transform.translation.1 * draw_region.height / 16.;
    }
}

#[cfg(debug_assertions)]
fn draw_draw_region_outline(
    mut toggle: Local<bool>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut gizmos: Gizmos,
    draw_region: Res<DrawRegion>,
) {
    if keyboard.just_pressed(KeyCode::Backquote) {
        *toggle ^= true;
    }
    if !*toggle {
        return;
    }

    gizmos
        .grid_2d(
            Vec2::ZERO,
            0.,
            UVec2::new(9, 16),
            vec2(draw_region.width / 9., draw_region.height / 16.),
            Color::srgb(1., 0., 0.),
        )
        .outer_edges();
}
