use bevy::app::{App, First, Plugin};
use bevy::log::LogPlugin;

use crate::cache_system::cache_frame_count;
use crate::config::FrameCountSubscriberConfig;
use crate::subscriber::add_log_plugin_with_custom_layer;

pub struct FrameCountSubscriberPlugin;

impl Plugin for FrameCountSubscriberPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, cache_frame_count)
            .init_resource::<FrameCountSubscriberConfig>();
        if !app.get_added_plugins::<LogPlugin>().is_empty() {
            panic!("LogPlugin already loaded, please disable with `.add_plugins(DefaultPlugins.build().disable::<LogPlugin>())` before loading FrameCountSubscriberPlugin");
        }
        if !app.get_added_plugins::<FrameCountSubscriberPlugin>().is_empty() {
            panic!("FrameCountSubscriberPlugin already loaded");
        }
        add_log_plugin_with_custom_layer(app);
    }
}
