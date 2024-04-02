use bevy::prelude::*;
use some_bevy_tools::range;

#[derive(Default)]
pub struct HealthMarker;

pub type Health = range::Range<HealthMarker>;
//pub type DeathEvent = range::StartRangeLimitReachedEvent<HealthMarker>;
//pub type FullHealEvent = range::EndRangeLimitReachedEvent<HealthMarker>;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(range::RangePlugin::<HealthMarker>::default());
    }
}
