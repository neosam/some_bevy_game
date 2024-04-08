use bevy::prelude::*;
use some_bevy_tools::{audio_loop::LoopableAudioSource, loading};

#[derive(Resource, Default, Reflect, Clone)]
pub struct ImageAssets {
    pub ship: Handle<Image>,
    pub wall: Handle<Image>,
    pub logo_overlay: Handle<Image>,
    pub bullet: Handle<Image>,
    pub rock: Handle<Image>,
}
impl loading::EasyAssetLoader for ImageAssets {
    type AssetType = Image;
    fn asset_mapper() -> &'static [(&'static str, &'static str)] {
        &[
            ("ship", "ship.png"),
            ("wall", "wall.png"),
            ("logo_overlay", "logo-overlay.png"),
            ("bullet", "bullet.png"),
            ("rock", "rock.png"),
        ]
    }
}

#[derive(Resource, Default, Reflect, Clone)]
pub struct MusicAssets {
    pub space: Handle<LoopableAudioSource>,
}
impl loading::EasyAssetLoader for MusicAssets {
    type AssetType = LoopableAudioSource;
    fn asset_mapper() -> &'static [(&'static str, &'static str)] {
        &[("space", "space.ogg")]
    }
}
