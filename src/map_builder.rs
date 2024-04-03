use crate::assets::ImageAssets;
use crate::{physics, GameState};
use bevy::prelude::*;
use some_bevy_tools::despawn;
use uuid::Uuid;

#[derive(Component)]
pub struct TileMarker(Uuid);

#[derive(Clone, Copy)]
pub enum TileType {
    Wall,
}

struct Tile {
    x: i32,
    y: i32,
    tile_type: TileType,
}

impl Tile {
    pub fn spawn_tile(
        &self,
        commands: &mut Commands,
        id: Uuid,
        image_assets: &ImageAssets,
        center: Vec2,
    ) {
        let position = Vec2::new(self.x as f32 * 50.0, self.y as f32 * 50.0) + center;
        let position = Vec3::new(position.x, position.y, 0.0);
        let image = match self.tile_type {
            TileType::Wall => image_assets.wall.clone(),
        };
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
            physics::PysicsBundle::fixed_rectangle(50.0, 50.0),
            TileMarker(id),
        ));
    }
}

pub struct Map {
    tiles: Vec<Tile>,
    pub id: Uuid,
}

impl Map {
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

pub struct MapDraft {
    pub width: u32,
    pub tiles: Vec<Option<TileType>>,
}

impl MapDraft {
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

    pub fn _get_tile(&self, x: u32, y: u32) -> Option<TileType> {
        self.tiles[self.pos_to_index(x, y)]
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile_type: TileType) {
        let index = self.pos_to_index(x, y);
        self.tiles[index] = Some(tile_type);
    }

    pub fn to_map(&self, center: (i32, i32)) -> Map {
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

pub fn build_corridor() -> Map {
    let mut draft = MapDraft::new(25, 25);
    for y in 0..22 {
        draft.set_tile(4, y, TileType::Wall);
    }
    for y in 0..25 {
        draft.set_tile(0, y, TileType::Wall);
    }
    for x in 0..5 {
        draft.set_tile(x, 0, TileType::Wall);
    }
    for x in 0..25 {
        draft.set_tile(x, 24, TileType::Wall);
    }
    for x in 5..25 {
        draft.set_tile(x, 21, TileType::Wall);
    }

    draft.to_map((2, 2))
}
