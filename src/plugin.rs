use crate::cache_system::cache_frame_count;
use crate::config::FrameCountSubscriberConfig;
use crate::subscriber::register_subscriber;
use bevy::app::{App, First, Plugin};

pub struct FrameCountSubscriberPlugin;

impl Plugin for FrameCountSubscriberPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, cache_frame_count)
            .init_resource::<FrameCountSubscriberConfig>();
        register_subscriber(app.world.get_resource::<FrameCountSubscriberConfig>());
    }
}
