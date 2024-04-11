use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
use bevy::sprite::MaterialMesh2dBundle;
#[cfg(target_arch = "wasm32")]
use bevy::window::WindowMode;
use bevy_rapier2d::prelude::*;
use bullet::BulletPlugin;
use ship::ship_orientation;
use ship::TutorialTrigger;
use some_bevy_tools::audio_loop::AudioLoopEvent;
use some_bevy_tools::audio_loop::AudioLoopPlugin;
use some_bevy_tools::camera_2d;
use some_bevy_tools::controller_2d;
use some_bevy_tools::despawn;
use some_bevy_tools::health;
use some_bevy_tools::input;
use some_bevy_tools::loading;
use some_bevy_tools::physics2d;
use some_bevy_tools::trigger;
use stars::StarMaterialSettings;

mod assets;
mod bullet;
mod map_builder;
mod maps;
mod ship;
mod stars;

fn main() {
    let mut app = App::new();
    app.insert_resource(RapierConfiguration {
        gravity: Vec2::new(0.0, 0.0),
        ..Default::default()
    })
    .insert_resource(StarMaterialSettings::default())
    .init_resource::<InGameState>()
    .insert_resource(AssetMetaCheck::Never);
    // Enable fullscreen in wasm
    #[cfg(target_arch = "wasm32")]
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Some Bevy Game".to_string(),
            //mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        }),
        ..Default::default()
    }));
    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Some Bevy Game".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugins(loading::LoadingPlugin(
            GameState::Loading,
            GameState::InGame,
        ))
        .add_plugins(loading::LoadPluginAssets(
            assets::ImageAssets::default(),
            GameState::Loading,
        ))
        .add_plugins(loading::LoadPluginAssets(
            assets::MusicAssets::default(),
            GameState::Loading,
        ))
        .add_plugins(despawn::CleanupPlugin(GameState::InGame))
        .add_plugins(camera_2d::Camera2DPlugin)
        .add_plugins(controller_2d::TopDownControllerPlugin)
        .add_plugins(Material2dPlugin::<stars::StarMaterial>::default())
        .add_plugins(trigger::PhysicsTriggerPlugin::<ship::Ship, TutorialTrigger>::default())
        .add_plugins(AudioLoopPlugin)
        .add_plugins(BulletPlugin)
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::InGame), (startup_ingame, show_logo))
        .add_systems(
            Update,
            (
                ship_orientation,
                user_event_handler,
                ship::tutorial_trigger_system,
                physics2d::acceleration_controller,
                stars::update_stars,
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

pub fn startup_ingame(
    mut commands: Commands,
    image_assets: Res<assets::ImageAssets>,
    music_assets: Res<assets::MusicAssets>,
    mut audio_events: EventWriter<AudioLoopEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<stars::StarMaterial>>,
) {
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
                physics_bundle: physics2d::PhysicsBundle::dynamic_rectangle(50.0, 50.0),
                acceleration: physics2d::Acceleration::new(1000.0, 300.0),
                direction: ship::Direction::Up,
                health: health::Health::new(0.0, 100.0),
                ship: ship::Ship,
            },
            despawn::Cleanup(GameState::InGame),
            controller_2d::SimpleTopDownController::new(10.0),
            ship::Player,
        ))
        .id();

    maps::tutorial::build_tutorial().spawn_tiles(&mut commands, &image_assets, Vec2::ZERO);

    let star_material = materials.add(stars::StarMaterial::default());

    commands
        .spawn((
            Camera2dBundle::default(),
            despawn::Cleanup(GameState::InGame),
            camera_2d::Camera2DController::new_follow_with_speed(player, 100.0),
            InheritedVisibility::VISIBLE,
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Stars".to_string()),
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(Rectangle {
                            half_size: Vec2::new(1280.0, 1280.0),
                        }))
                        .into(),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(1280.0))
                        .with_translation(Vec3::new(0.0, 0.0, -1.0)),
                    material: star_material.clone(),
                    ..default()
                },
            ));
        });

    commands.spawn(AudioSourceBundle {
        source: music_assets.space.clone(),
        ..default()
    });
    audio_events.send(AudioLoopEvent::EndPositionImmediate(
        19.2,
        music_assets.space.clone(),
    ));
}

#[derive(Component, Default)]
pub struct StaticWall;

#[derive(Resource, Default)]
pub struct InGameState {
    pub block_controls: bool,
}

fn user_event_handler(
    mut controller_events: EventReader<input::ActionEvent<controller_2d::TopDownAction>>,
    mut bullet_events: EventWriter<bullet::ShootBullet>,
    mut query: Query<
        (Entity, &mut physics2d::Acceleration, &mut ship::Direction),
        With<controller_2d::SimpleTopDownController>,
    >,
    in_game_state: Res<InGameState>,
    mut next_shoot_time: Local<f32>,
    time: Res<Time>,
) {
    if in_game_state.block_controls {
        return;
    }
    if let Ok((ship_entity, mut acceleration, mut direction)) = query.get_single_mut() {
        acceleration.direction = physics2d::AccelerationDirection::None;
        for action in controller_events.read() {
            match action.action {
                controller_2d::TopDownAction::MoveUp => {
                    acceleration.direction = physics2d::AccelerationDirection::Up;
                    *direction = ship::Direction::Up;
                }
                controller_2d::TopDownAction::MoveDown => {
                    acceleration.direction = physics2d::AccelerationDirection::Down;
                    *direction = ship::Direction::Down;
                }
                controller_2d::TopDownAction::MoveLeft => {
                    acceleration.direction = physics2d::AccelerationDirection::Left;
                    *direction = ship::Direction::Left;
                }
                controller_2d::TopDownAction::MoveRight => {
                    acceleration.direction = physics2d::AccelerationDirection::Right;
                    *direction = ship::Direction::Right;
                }
                controller_2d::TopDownAction::Action => {
                    if time.elapsed_seconds() > *next_shoot_time {
                        bullet_events.send(bullet::ShootBullet { ship: ship_entity });
                        *next_shoot_time = time.elapsed_seconds() + 0.5;
                    }
                }
                _ => {}
            }
        }
    }
}

#[derive(Component)]
pub struct Logo;

pub fn show_logo(mut commands: Commands, image_assets: Res<assets::ImageAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    image: UiImage::new(image_assets.logo_overlay.clone()),
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Logo,
            ));
        });
}
