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
}
