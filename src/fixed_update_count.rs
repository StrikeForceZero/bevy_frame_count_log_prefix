use bevy_app::{App, FixedPostUpdate, Plugin};
use bevy_ecs::prelude::{ResMut, Resource};

#[derive(Debug, Default, Resource, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FixedUpdateCount(pub u32);

#[derive(Default)]
pub struct FixedUpdateCountPlugin;

impl Plugin for FixedUpdateCountPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FixedUpdateCount>();
        app.add_systems(FixedPostUpdate, update_fixed_update_count);
    }
}

/// A system used to increment [`FixedUpdateCount`] with wrapping addition.
pub fn update_fixed_update_count(mut fixed_update_count: ResMut<FixedUpdateCount>) {
    fixed_update_count.0 = fixed_update_count.0.wrapping_add(1);
}