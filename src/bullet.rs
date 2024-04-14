use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use some_bevy_tools::{
    collision_detection::{self, CollisionEventStart},
    despawn::{AutoDespawn, AutoDespawnPlugin},
    health::{Health, HealthPlugin},
    physics2d::{self, PhysicsBundle},
};

use crate::{
    assets::ImageAssets,
    ship::{self, Ship},
    StaticWall,
};

#[derive(PartialEq, Eq, Default)]
pub enum DamagerType {
    #[default]
    SelfDestruct,
}

#[derive(Component, Default)]
pub struct Damager {
    pub damager_type: DamagerType,
    pub strength: f32,
}

impl Damager {
    pub fn new_self_destruct(strength: f32) -> Self {
        Self {
            damager_type: DamagerType::SelfDestruct,
            strength,
        }
    }
}

pub fn damage_system(
    mut commands: Commands,
    mut damage_collisions: EventReader<
        some_bevy_tools::collision_detection::CollisionEventStart<
            some_bevy_tools::health::Health,
            Damager,
        >,
    >,
    damager_query: Query<&Damager>,
    mut health_query: Query<&mut Health, Without<Damager>>,
) {
    for CollisionEventStart(health_entity, damager_entity, _) in damage_collisions.read() {
        if let (Ok(damager), Ok(mut health)) = (
            damager_query.get(*damager_entity),
            health_query.get_mut(*health_entity),
        ) {
            bevy::log::info!("Damager causes damage: {}", health.get());
            health.modify(-damager.strength);
            if damager.damager_type == DamagerType::SelfDestruct {
                commands
                    .entity(*damager_entity)
                    .insert(some_bevy_tools::despawn::AutoDespawn::with_duration(0.1));
            }
        }
    }
}

pub struct DamagerPlugin;
impl Plugin for DamagerPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<HealthPlugin>() {
            app.add_plugins(HealthPlugin);
        }
        if !app.is_plugin_added::<some_bevy_tools::collision_detection::CollisionDetectionPlugin<Health, Damager>>() {
            app.add_plugins(collision_detection::CollisionDetectionPlugin::<Health, Damager>::default());
        }
        app.add_systems(Update, damage_system);
    }
}

pub fn despawn_the_dead(
    mut commands: Commands,
    mut death_events: EventReader<some_bevy_tools::health::DeathEvent>,
) {
    for event in death_events.read() {
        bevy::log::info!("Something died");
        commands.entity(event.entity).despawn_recursive();
    }
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub sprite_bundle: SpriteBundle,
    pub physics_bundle: physics2d::PhysicsBundle,
    pub damage: Damager,
    pub auto_despawn: some_bevy_tools::despawn::AutoDespawn,
}
impl BulletBundle {
    pub fn new(position: Vec2, image_assets: &ImageAssets, velocity: Vec2, strength: f32) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                texture: image_assets.bullet.clone(),
                transform: Transform::from_translation(position.extend(0.0)),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                ..default()
            },
            physics_bundle: PhysicsBundle {
                velocity: Velocity::linear(velocity),
                collider: Collider::cuboid(5.0, 5.0),
                ..physics2d::PhysicsBundle::dynamic_rectangle(50.0, 50.0)
            },
            damage: Damager::new_self_destruct(strength),
            auto_despawn: some_bevy_tools::despawn::AutoDespawn::with_duration(5.0),
        }
    }
}

#[derive(Event)]
pub struct ShootBullet {
    pub ship: Entity,
}

pub fn shoot_bullet_system(
    mut commands: Commands,
    mut events: EventReader<ShootBullet>,
    image_assets: Res<ImageAssets>,
    ship_query: Query<(&Transform, &Velocity, &ship::Direction), With<Ship>>,
) {
    for event in events.read() {
        if let Ok((transform, velocity, direction)) = ship_query.get(event.ship) {
            let position = transform.translation.xy() + direction.vector() * 50.0;
            let velocity = velocity.linvel + direction.vector() * 100.0;
            commands.spawn(BulletBundle::new(position, &image_assets, velocity, 10.0));
        }
    }
}

pub fn despawn_bullet_on_collision<T: Component>(
    mut commands: Commands,
    mut wall_collider: EventReader<
        some_bevy_tools::collision_detection::CollisionEventStart<T, Damager>,
    >,
) {
    for CollisionEventStart(_, damager_entity, _) in wall_collider.read() {
        commands
            .entity(*damager_entity)
            .insert(AutoDespawn::with_duration(0.1));
    }
}

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<DamagerPlugin>() {
            app.add_plugins(DamagerPlugin);
        }
        if !app.is_plugin_added::<AutoDespawnPlugin>() {
            app.add_plugins(AutoDespawnPlugin);
        }
        app
            .add_plugins(
                some_bevy_tools::collision_detection::CollisionDetectionPlugin::<
                    StaticWall,
                    Damager,
                >::default()
            )
            .add_event::<ShootBullet>()
            .add_systems(
                Update,
                (
                    shoot_bullet_system,
                    despawn_bullet_on_collision::<StaticWall>,
                    despawn_the_dead,
                ),
            );
    }
}
