use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use ship::ship_orientation;
use some_bevy_tools::camera_2d;
use some_bevy_tools::controller_2d;
use some_bevy_tools::despawn;
use some_bevy_tools::input;
use some_bevy_tools::loading;

mod assets;
mod health;
mod physics;
mod ship;

fn main() {
    App::new()
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, 0.0),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugins(loading::LoadingPlugin(
            GameState::Loading,
            GameState::InGame,
        ))
        .add_plugins(loading::LoadPluginAssets(
            assets::ImageAssets::default(),
            GameState::Loading,
        ))
        .add_plugins(despawn::CleanupPlugin(GameState::InGame))
        .add_plugins(camera_2d::Camera2DPlugin)
        .add_plugins(controller_2d::TopDownControllerPlugin)
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::InGame), startup_ingame)
        .add_systems(
            Update,
            (
                ship_orientation,
                user_event_handler,
                physics::acceleration_controller,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .run();
}

#[derive(States, PartialEq, Eq, Debug, Default, Hash, Clone, Copy)]
pub enum GameState {
    #[default]
    Loading,
    InGame,
}

pub fn startup_ingame(mut commands: Commands, image_assets: Res<assets::ImageAssets>) {
    let player = commands
        .spawn((
            ship::ShipBundle {
                sprite_bundle: SpriteBundle {
                    texture: image_assets.ship.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(50.0, 50.0)),
                        ..default()
                    },
                    ..default()
                },
                physics_bundle: physics::PysicsBundle::dynamic_rectangle(50.0, 50.0),
                acceleration: physics::Acceleration::new(1000.0, 300.0),
                direction: ship::Direction::Up,
                health: health::Health::new(0.0, 100.0),
            },
            despawn::Cleanup(GameState::InGame),
            controller_2d::SimpleTopDownController::new(10.0),
        ))
        .id();

    commands.spawn((
        SpriteBundle {
            texture: image_assets.wall.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(200.0, 0.0, 0.0),
            ..default()
        },
        despawn::Cleanup(GameState::InGame),
        physics::PysicsBundle::fixed_rectangle(50.0, 50.0),
    ));

    commands.spawn((
        Camera2dBundle::default(),
        despawn::Cleanup(GameState::InGame),
        camera_2d::Camera2DController::new_follow_with_speed(player, 100.0),
    ));
}

fn user_event_handler(
    mut controller_events: EventReader<input::ActionEvent<controller_2d::TopDownAction>>,
    mut query: Query<
        (&mut physics::Acceleration, &mut ship::Direction),
        With<controller_2d::SimpleTopDownController>,
    >,
) {
    if let Ok((mut acceleration, mut direction)) = query.get_single_mut() {
        acceleration.direction = physics::AccelerationDirection::None;
        for action in controller_events.read() {
            match action.action {
                controller_2d::TopDownAction::MoveUp => {
                    acceleration.direction = physics::AccelerationDirection::Up;
                    *direction = ship::Direction::Up;
                }
                controller_2d::TopDownAction::MoveDown => {
                    acceleration.direction = physics::AccelerationDirection::Down;
                    *direction = ship::Direction::Down;
                }
                controller_2d::TopDownAction::MoveLeft => {
                    acceleration.direction = physics::AccelerationDirection::Left;
                    *direction = ship::Direction::Left;
                }
                controller_2d::TopDownAction::MoveRight => {
                    acceleration.direction = physics::AccelerationDirection::Right;
                    *direction = ship::Direction::Right;
                }
                _ => {}
            }
        }
    }
}
