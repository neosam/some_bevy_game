use crate::health;
use crate::velocity;
use bevy::prelude::*;

#[derive(Component)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Bundle)]
pub struct ShipBundle {
    pub movement_bundle: velocity::MovementBundle,
    pub sprite_bundle: SpriteBundle,

    pub direction: Direction,
    pub health: health::Health,
}

pub fn ship_orientation(mut query: Query<(&Direction, &mut Transform)>) {
    for (direction, mut transform) in query.iter_mut() {
        match direction {
            Direction::Left => {
                transform.rotation = Quat::from_rotation_z(std::f32::consts::PI / 2.0);
            }
            Direction::Right => {
                transform.rotation = Quat::from_rotation_z(-std::f32::consts::PI / 2.0);
            }
            Direction::Up => {
                transform.rotation = Quat::from_rotation_z(0.0);
            }
            Direction::Down => {
                transform.rotation = Quat::from_rotation_z(std::f32::consts::PI);
            }
        }
    }
}
