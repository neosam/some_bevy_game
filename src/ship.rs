use crate::{assets, health, stars, InGameState};
use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;
use some_bevy_tools::{
    audio_loop::AudioLoopEvent, collision_detection::CollisionEventStart, physics2d,
};

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
    pub physics_bundle: physics2d::PhysicsBundle,

    pub acceleration: physics2d::Acceleration,

    pub direction: Direction,
    pub health: health::Health,
    pub ship: Ship,
}
#[derive(Component)]
pub struct Player;

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
    DeepSpace,
}

pub fn tutorial_trigger_system(
    mut turtorial_trigger1: EventReader<
        some_bevy_tools::collision_detection::CollisionEventStart<Ship, TutorialTrigger>,
    >,
    query: Query<&TutorialTrigger>,
    music_assets: Res<assets::MusicAssets>,
    mut audio_events: EventWriter<AudioLoopEvent>,
    mut stars_materials: ResMut<stars::StarMaterialSettings>,
    mut in_game_state: ResMut<InGameState>,
    mut ship_direction: Query<(&mut Direction, &mut Velocity), (With<Ship>, With<Player>)>,
    mut camera_query: Query<&mut some_bevy_tools::camera_2d::Camera2DController>,
) {
    for CollisionEventStart(_, trigger, _) in turtorial_trigger1.read() {
        let trigger = query.get(*trigger).unwrap();
        match trigger {
            TutorialTrigger::SimplyForward => {
                bevy::log::info!("SimplyForward");
                audio_events.send(AudioLoopEvent::LoopOffsetImmediate(
                    19.2,
                    music_assets.space.clone(),
                ));
            }
            TutorialTrigger::TurnedRight => {
                bevy::log::info!("TurnedRight");
                audio_events.send_batch([
                    AudioLoopEvent::StartPositionImmediate(19.2, music_assets.space.clone()),
                    AudioLoopEvent::EndPositionImmediate(19.2 * 4.0, music_assets.space.clone()),
                ]);
            }
            TutorialTrigger::DeepSpace => {
                bevy::log::info!("DeepSpace");
                stars_materials.desired_speed_x = 10000.0;
                stars_materials.acceleration = 2000.0;
                in_game_state.block_controls = true;
                let (mut direction, mut velocity) = ship_direction.get_single_mut().unwrap();
                *direction = Direction::Right;
                velocity.linvel.x = 300.0;
                velocity.linvel.y = 0.0;

                let mut camera_controller = camera_query.get_single_mut().unwrap();
                camera_controller.mode = some_bevy_tools::camera_2d::Camera2DMode::Move;
                camera_controller.speed = 310.0;
            }
        }
    }
}
