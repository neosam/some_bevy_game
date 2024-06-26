use crate::GameState;
use crate::{assets::ImageAssets, StaticWall};
use bevy::prelude::*;
use core::marker::Copy;
use some_bevy_tools::health::Health;
use some_bevy_tools::{despawn, physics2d, trigger};
use thiserror::Error;
use uuid::Uuid;

#[derive(Component)]
#[allow(dead_code)]
pub struct TileMarker(Uuid);

#[derive(Clone, Copy)]
pub enum TileType<T: Clone + Copy> {
    Wall,
    Rock,
    _Trigger(T, f32),
    SingleTrigger(T, f32),
}

struct Tile<T: Clone + Copy> {
    x: i32,
    y: i32,
    tile_type: TileType<T>,
}

enum TileInfo<T: Clone + Copy> {
    StaticImage(Handle<Image>),
    HealthImage(Handle<Image>, f32),
    Trigger(T, f32),
    SingleTrigger(T, f32),
}

impl<T: Clone + Copy + Component> Tile<T> {
    pub fn spawn_tile(
        &self,
        commands: &mut Commands,
        id: Uuid,
        image_assets: &ImageAssets,
        center: Vec2,
    ) {
        let position = Vec2::new(self.x as f32 * 50.0, self.y as f32 * 50.0) + center;
        let position = Vec3::new(position.x, position.y, 0.0);
        let tile_info = match self.tile_type {
            TileType::Wall => TileInfo::StaticImage(image_assets.wall.clone()),
            TileType::Rock => TileInfo::HealthImage(image_assets.rock.clone(), 10.0),
            TileType::_Trigger(trigger, size_multiplier) => {
                TileInfo::Trigger(trigger, size_multiplier)
            }
            TileType::SingleTrigger(trigger, size_multiplier) => {
                TileInfo::SingleTrigger(trigger, size_multiplier)
            }
        };
        match tile_info {
            TileInfo::StaticImage(image) => {
                commands.spawn((
                    SpriteBundle {
                        texture: image,
                        transform: Transform::from_translation(position),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(50.0, 50.0)),
                            ..default()
                        },
                        ..default()
                    },
                    despawn::Cleanup(GameState::InGame),
                    physics2d::PhysicsBundle::fixed_rectangle(50.0, 50.0),
                    TileMarker(id),
                    StaticWall,
                ));
            }
            TileInfo::HealthImage(image, health) => {
                commands.spawn((
                    SpriteBundle {
                        texture: image,
                        transform: Transform::from_translation(position),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(50.0, 50.0)),
                            ..default()
                        },
                        ..default()
                    },
                    despawn::Cleanup(GameState::InGame),
                    physics2d::PhysicsBundle::fixed_rectangle(50.0, 50.0),
                    TileMarker(id),
                    Health::new(0.0, health),
                ));
            }
            TileInfo::Trigger(trigger, size_multiplier) => {
                commands.spawn((
                    physics2d::PhysicsBundle::trigger(50.0, 50.0, size_multiplier),
                    trigger,
                    Transform::from_translation(position),
                    GlobalTransform::default(),
                ));
            }
            TileInfo::SingleTrigger(trigger, size_multiplier) => {
                commands.spawn((
                    physics2d::PhysicsBundle::trigger(50.0, 50.0, size_multiplier),
                    trigger::SingleTrigger,
                    trigger,
                    Transform::from_translation(position),
                    GlobalTransform::default(),
                ));
            }
        }
    }
}

pub struct Map<T: Clone + Copy> {
    tiles: Vec<Tile<T>>,
    pub id: Uuid,
}

impl<T: Clone + Copy + Component> Map<T> {
    pub fn new() -> Self {
        Self {
            tiles: Vec::new(),
            id: Uuid::new_v4(),
        }
    }

    pub fn spawn_tiles(&self, commands: &mut Commands, image_assets: &ImageAssets, center: Vec2) {
        for tile in &self.tiles {
            tile.spawn_tile(commands, self.id, image_assets, center);
        }
    }
}

#[derive(Error, Debug)]
pub enum MapDraftError {
    #[error("str width is inconsistent. Every &str of a [&str] must have the same length")]
    InconsistentWidth,

    #[error("str length does not match, must be width times height long")]
    StrLengthMismatch,
}

pub struct MapDraft<T: Clone + Copy> {
    pub width: u32,
    pub tiles: Vec<Option<TileType<T>>>,
}

impl<T: Clone + Copy + Component> MapDraft<T> {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            tiles: vec![None; (width * height) as usize],
        }
    }

    pub fn pos_to_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn index_to_pos(&self, index: usize) -> (u32, u32) {
        (index as u32 % self.width, index as u32 / self.width)
    }

    pub fn _get_tile(&self, x: u32, y: u32) -> Option<TileType<T>> {
        self.tiles[self.pos_to_index(x, y)]
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile_type: TileType<T>) {
        let index = self.pos_to_index(x, y);
        self.tiles[index] = Some(tile_type);
    }

    pub fn to_map(&self, center: (i32, i32)) -> Map<T> {
        let mut map = Map::new();
        for (index, tile) in self.tiles.iter().enumerate() {
            if let Some(tile) = tile {
                let (x, y) = self.index_to_pos(index);
                map.tiles.push(Tile {
                    x: x as i32 - center.0,
                    y: y as i32 - center.1,
                    tile_type: *tile,
                });
            }
        }
        map
    }

    pub fn from_str(
        map_str: &str,
        width: u32,
        height: u32,
        tile_mapper: Box<dyn Fn(char) -> Option<TileType<T>>>,
    ) -> Result<MapDraft<T>, MapDraftError> {
        if map_str.len() as u32 != width * height {
            return Err(MapDraftError::StrLengthMismatch);
        }
        let mut draft = MapDraft::new(width, height);
        for (i, c) in map_str.chars().enumerate() {
            let x = i as u32 % width;
            let y = height - 1 - i as u32 / width;
            if let Some(tile) = tile_mapper(c) {
                draft.set_tile(x, y, tile);
            }
        }
        Ok(draft)
    }

    pub fn from_str_array(
        array: &[&str],
        tile_mapper: Box<dyn Fn(char) -> Option<TileType<T>>>,
    ) -> Result<MapDraft<T>, MapDraftError> {
        let width = array[0].len() as u32;
        let height = array.len() as u32;
        let mut draft = MapDraft::new(width, height);
        for (y, line) in array.iter().rev().enumerate() {
            if line.len() as u32 != width {
                return Err(MapDraftError::InconsistentWidth);
            }
            for (x, c) in line.chars().enumerate() {
                if let Some(tile) = tile_mapper(c) {
                    draft.set_tile(x as u32, y as u32, tile);
                }
            }
        }
        Ok(draft)
    }
}
