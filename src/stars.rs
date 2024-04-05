use bevy::{prelude::*, render::render_resource::AsBindGroup, sprite::Material2d};

/// Star material
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone, Default)]
pub struct StarMaterial {
    #[uniform(100)]
    pub relative_pos_x: f32,
    #[uniform(100)]
    pub relative_pos_y: f32,
    #[uniform(100)]
    pub _padding1: f32,
    #[uniform(100)]
    pub _padding2: f32,
}
impl Material2d for StarMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/extended_2d_material.wgsl".into()
    }
}

/// Resource which contains additional information about the star material
/// It contains it's speed.
#[derive(Resource, Default)]
pub struct StarMaterialSettings {
    pub speed_x: f32,
    pub speed_y: f32,
    pub desired_speed_x: f32,
    pub desired_speed_y: f32,
    pub acceleration: f32,
}

/// A system which updates the speed.
pub(crate) fn update_stars(
    time: Res<Time>,
    mut materials: ResMut<Assets<StarMaterial>>,
    mut settings: ResMut<StarMaterialSettings>,
) {
    /* Expect only one StarMaterial */
    let star_material_id = materials.ids().next().unwrap();
    let star_material = materials.get_mut(star_material_id).unwrap();
    if settings.speed_x < settings.desired_speed_x {
        settings.speed_x += settings.acceleration * time.delta_seconds();
    }
    if settings.speed_x > settings.desired_speed_x {
        settings.speed_x -= settings.acceleration * time.delta_seconds();
    }
    if settings.speed_y < settings.desired_speed_y {
        settings.speed_y += settings.acceleration * time.delta_seconds();
    }
    if settings.speed_y > settings.desired_speed_y {
        settings.speed_y -= settings.acceleration * time.delta_seconds();
    }
    star_material.relative_pos_x += settings.speed_x * time.delta_seconds();
    star_material.relative_pos_y += settings.speed_y * time.delta_seconds();
}
