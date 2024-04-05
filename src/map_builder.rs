use crate::assets::ImageAssets;
use crate::GameState;
use bevy::prelude::*;
use core::marker::Copy;
use some_bevy_tools::{despawn, physics2d, trigger};
use uuid::Uuid;

#[derive(Component)]
#[allow(dead_code)]
pub struct TileMarker(Uuid);

#[derive(Clone, Copy)]
pub enum TileType<T: Clone + Copy> {
    Wall,
    _Trigger(T, f32),
    SingleTrigger(T, f32),
}

struct Tile<T: Clone + Copy> {
    x: i32,
    y: i32,
    tile_type: TileType<T>,
}

enum TileInfo<T: Clone + Copy> {
    Image(Handle<Image>),
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
            TileType::Wall => TileInfo::Image(image_assets.wall.clone()),
            TileType::_Trigger(trigger, size_multiplier) => {
                TileInfo::Trigger(trigger, size_multiplier)
            }
            TileType::SingleTrigger(trigger, size_multiplier) => {
                TileInfo::SingleTrigger(trigger, size_multiplier)
            }
        };
        match tile_info {
            TileInfo::Image(image) => {
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
}

pub fn build_corridor<T: Clone + Copy + Component>(trigger1: T, trigger2: T) -> Map<T> {
    let map = "XXXXXXXXXXXXXXXXXXXXXXXXXXXX \
                     X                          X \
                     X 1                      2 X \
                     X   XXXXXXXXXXXXXXXXXXXXX  X \
                     X   X                  XX  X \
                     X   X                  X   XX\
                     X   X                  X    B\
                     X   X                  X    B\
                     X   X                  XXXXXX\
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     XXXXX                        ";

    let mut draft = MapDraft::new(29, 25);
    for (i, c) in map.chars().enumerate() {
        match c {
            _ if c == 'X' => draft.set_tile(i as u32 % 29, 24 - i as u32 / 29, TileType::Wall),
            _ if c == '1' => draft.set_tile(
                i as u32 % 29,
                25 - i as u32 / 29,
                TileType::SingleTrigger(trigger1, 1.1),
            ),
            _ if c == '2' => draft.set_tile(
                i as u32 % 29,
                25 - i as u32 / 29,
                TileType::SingleTrigger(trigger2, 1.1),
            ),
            _ => (),
        }
    }

    draft.to_map((2, 2))
}
