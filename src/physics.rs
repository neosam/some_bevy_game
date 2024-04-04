use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
pub struct SingleTrigger;

#[derive(Bundle)]
pub struct PhysicsBundle {
    pub velocity: Velocity,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub active_events: ActiveEvents,
    pub locked_axes: LockedAxes,
}

impl PhysicsBundle {
    pub fn dynamic_rectangle(width: f32, height: f32) -> Self {
        PhysicsBundle {
            velocity: Velocity::zero(),
            collider: Collider::cuboid(width / 2.0, height / 2.0),
            rigid_body: RigidBody::Dynamic,
            active_events: ActiveEvents::COLLISION_EVENTS,
            locked_axes: LockedAxes::ROTATION_LOCKED,
        }
    }

    pub fn fixed_rectangle(width: f32, height: f32) -> Self {
        PhysicsBundle {
            velocity: Velocity::zero(),
            collider: Collider::cuboid(width / 2.0, height / 2.0),
            rigid_body: RigidBody::Fixed,
            active_events: ActiveEvents::COLLISION_EVENTS,
            locked_axes: LockedAxes::ROTATION_LOCKED,
        }
    }

    pub fn trigger(width: f32, height: f32, size_multiplier: f32) -> (Self, Sensor) {
        (
            PhysicsBundle {
                velocity: Velocity::zero(),
                collider: Collider::cuboid(
                    width / 2.0 * size_multiplier,
                    height / 2.0 * size_multiplier,
                ),
                rigid_body: RigidBody::Dynamic,
                active_events: ActiveEvents::COLLISION_EVENTS,
                locked_axes: LockedAxes::ROTATION_LOCKED,
            },
            Sensor,
        )
    }
}

#[derive(Default)]
pub enum AccelerationDirection {
    #[default]
    None,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Default)]
pub struct Acceleration {
    pub amount: f32,
    pub max_speed: f32,
    pub direction: AccelerationDirection,
}
impl Acceleration {
    pub fn new(amount: f32, max_speed: f32) -> Self {
        Acceleration {
            amount,
            max_speed,
            ..Default::default()
        }
    }
}

pub fn acceleration_controller(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut velocity, acceleration) in query.iter_mut() {
        match acceleration.direction {
            AccelerationDirection::Up => {
                velocity.linvel.y += acceleration.amount * time.delta_seconds();
            }
            AccelerationDirection::Down => {
                velocity.linvel.y -= acceleration.amount * time.delta_seconds();
            }
            AccelerationDirection::Left => {
                velocity.linvel.x -= acceleration.amount * time.delta_seconds();
            }
            AccelerationDirection::Right => {
                velocity.linvel.x += acceleration.amount * time.delta_seconds();
            }
            _ => (),
        }
        velocity.linvel.x = velocity
            .linvel
            .x
            .clamp(-acceleration.max_speed, acceleration.max_speed);
        velocity.linvel.y = velocity
            .linvel
            .y
            .clamp(-acceleration.max_speed, acceleration.max_speed);
    }
}

pub fn remove_after_collision<T1: Component, T2: Component>(
    mut commands: Commands,
    mut collision_events: EventReader<
        some_bevy_tools::collision_detection::CollisionEventStop<T1, T2>,
    >,
    query: Query<Entity, With<T2>>,
) {
    for collision_event in collision_events.read() {
        if let Ok(entity) = query.get(collision_event.1) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
