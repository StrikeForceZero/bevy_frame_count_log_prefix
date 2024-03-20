use crate::cache_system::cache_frame_count;
use crate::subscriber::register_subscriber;
use bevy::app::{App, First, Plugin};

pub struct FrameCountSubscriberPlugin;

impl Plugin for FrameCountSubscriberPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, cache_frame_count);
        register_subscriber();
    }
}
