use bevy::prelude::*;
use some_bevy_tools::range;

#[derive(Default)]
pub struct VelocityX;
#[derive(Default)]
pub struct VelocityY;

pub enum AccelerationType {
    Left,
    Right,
    Up,
    Down,
    None,
}

#[derive(Component)]
pub struct Acceleration(pub f32, pub AccelerationType);

#[derive(Bundle)]
pub struct MovementBundle {
    pub velocity_x: range::Range<VelocityX>,
    pub velocity_y: range::Range<VelocityY>,
    pub acceleration: Acceleration,
}

impl MovementBundle {
    pub fn with_acceleration(max_speed: f32, acceleration: f32) -> Self {
        Self {
            velocity_x: range::Range::new(-max_speed, max_speed).with_current(0.0),
            velocity_y: range::Range::new(-max_speed, max_speed).with_current(0.0),
            acceleration: Acceleration(acceleration, AccelerationType::None),
        }
    }
}

pub fn acceleration_system(
    time: Res<Time>,
    mut query: Query<(
        &mut range::Range<VelocityX>,
        &mut range::Range<VelocityY>,
        &Acceleration,
    )>,
) {
    for (mut velocity_x, mut velocity_y, Acceleration(acceleration_amount, acceleration_type)) in
        query.iter_mut()
    {
        match acceleration_type {
            AccelerationType::Left => {
                velocity_x.modify(-acceleration_amount * time.delta_seconds());
            }
            AccelerationType::Right => {
                velocity_x.modify(acceleration_amount * time.delta_seconds());
            }
            AccelerationType::Up => {
                velocity_y.modify(acceleration_amount * time.delta_seconds());
            }
            AccelerationType::Down => {
                velocity_y.modify(-acceleration_amount * time.delta_seconds());
            }
            _ => {}
        }
    }
}

pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(
        &range::Range<VelocityX>,
        &range::Range<VelocityY>,
        &mut Transform,
    )>,
) {
    for (velocity_x, velocity_y, mut transform) in query.iter_mut() {
        transform.translation.x += velocity_x.get() * time.delta_seconds();
        transform.translation.y += velocity_y.get() * time.delta_seconds();
    }
}
