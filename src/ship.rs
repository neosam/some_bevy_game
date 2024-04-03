use crate::{health, physics};
use bevy::prelude::*;
use some_bevy_tools::collision_detection::CollisionEventStart;

#[derive(Component)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component, Default)]
pub struct Ship;

#[derive(Bundle)]
pub struct ShipBundle {
    pub sprite_bundle: SpriteBundle,
    pub physics_bundle: physics::PhysicsBundle,

    pub acceleration: physics::Acceleration,

    pub direction: Direction,
    pub health: health::Health,
    pub ship: Ship,
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

#[derive(Component, Clone, Copy, Default)]
pub enum TutorialTrigger {
    #[default]
    SimplyForward,
    TurnedRight,
}

pub fn tutorial_trigger_system(
    mut turtorial_trigger1: EventReader<
        some_bevy_tools::collision_detection::CollisionEventStart<Ship, TutorialTrigger>,
    >,
    query: Query<&TutorialTrigger>,
) {
    for CollisionEventStart(_, trigger, _) in turtorial_trigger1.read() {
        let trigger = query.get(*trigger).unwrap();
        match trigger {
            TutorialTrigger::SimplyForward => {
                println!("Tutorial trigger 1: SimplyForward");
            }
            TutorialTrigger::TurnedRight => {
                println!("Tutorial trigger 2: TurnedRight");
            }
        }
    }
}
