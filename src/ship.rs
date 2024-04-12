#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use std::time::Duration;

use crate::{assets, error_handler::GameError, health, maps, stars, InGameState, Logo};
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
impl Direction {
    pub fn vector(&self) -> Vec2 {
        match self {
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::Right => Vec2::new(1.0, 0.0),
            Direction::Up => Vec2::new(0.0, 1.0),
            Direction::Down => Vec2::new(0.0, -1.0),
        }
    }
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
    mut commands: Commands,
    image_assets: Res<assets::ImageAssets>,
    mut turtorial_trigger1: EventReader<
        some_bevy_tools::collision_detection::CollisionEventStart<Ship, TutorialTrigger>,
    >,
    query: Query<&TutorialTrigger>,
    music_assets: Res<assets::MusicAssets>,
    mut audio_events: EventWriter<AudioLoopEvent>,
    mut stars_materials: ResMut<stars::StarMaterialSettings>,
    mut in_game_state: ResMut<InGameState>,
    mut ship_direction: Query<
        (&mut Direction, &mut Velocity, &Transform),
        (With<Ship>, With<Player>),
    >,
    mut camera_query: Query<&mut some_bevy_tools::camera_2d::Camera2DController>,
    mut logo_query: Query<&mut Visibility, With<Logo>>,
    time: Res<Time>,
    mut logo_timer: Local<Option<Timer>>,
    mut logo_disappear_timer: Local<Option<Timer>>,
) -> Result<(), GameError> {
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
                let (mut direction, mut velocity, _) = ship_direction.get_single_mut().unwrap();
                *direction = Direction::Right;
                velocity.linvel.x = 300.0;
                velocity.linvel.y = 0.0;

                let mut camera_controller = camera_query.get_single_mut().unwrap();
                camera_controller.mode = some_bevy_tools::camera_2d::Camera2DMode::Move;
                camera_controller.speed = 310.0;

                *logo_timer = Some(Timer::new(Duration::from_secs(2), TimerMode::Once));
            }
        }
    }
    if let Some(timer) = logo_timer.as_mut() {
        if timer.tick(time.delta()).just_finished() {
            let mut logo_visibility = logo_query.get_single_mut().unwrap();
            *logo_visibility = Visibility::Visible;
            *logo_disappear_timer = Some(Timer::new(Duration::from_secs(7), TimerMode::Once));
        }
    }
    if let Some(timer) = logo_disappear_timer.as_mut() {
        if timer.tick(time.delta()).just_finished() {
            let (_, mut velocity, transform) = ship_direction.get_single_mut().unwrap();
            velocity.linvel.x = 0.0;
            velocity.linvel.y = 0.0;

            let mut logo_visibility = logo_query.get_single_mut().unwrap();
            *logo_visibility = Visibility::Hidden;
            maps::level_1::build_level_1()?.spawn_tiles(
                &mut commands,
                &image_assets,
                transform.translation.xy(),
            );

            let mut camera_controller = camera_query.get_single_mut().unwrap();
            camera_controller.mode = some_bevy_tools::camera_2d::Camera2DMode::Follow;
            in_game_state.block_controls = false;
            stars_materials.desired_speed_x = 0.0;
            stars_materials.acceleration = 20000.0;
        }
    }
    Ok(())
}
