use bevy::app::{App, First, Plugin};
use bevy::log::LogPlugin;

use crate::cache_system::cache_frame_count;
use crate::config::FrameCountSubscriberConfig;
use crate::subscriber_layer::add_log_plugin_with_custom_layer;

fn init(app: &mut App) {
    app.add_systems(First, cache_frame_count)
        .init_resource::<FrameCountSubscriberConfig>();
}

pub struct FrameCountSubscriberPlugin;

impl Plugin for FrameCountSubscriberPlugin {
    fn build(&self, app: &mut App) {
        init(app);
        if !app.get_added_plugins::<LogPlugin>().is_empty() {
            panic!("LogPlugin already loaded, please disable with `.add_plugins(DefaultPlugins.build().disable::<LogPlugin>())` before loading FrameCountSubscriberPlugin, \
             or add the layer manually `LogPlugin {{ custom_layer: |app| Some(Box::new(vec![/* ..other layers here, */ bevy_frame_count_subscriber::subscriber::frame_count_layer(app))) }}`");
        }
        if !app.get_added_plugins::<FrameCountSubscriberPlugin>().is_empty() {
            panic!("FrameCountSubscriberPlugin already loaded");
        }
        add_log_plugin_with_custom_layer(app);
    }
}

pub struct FrameCountSubscriberPluginManual;

impl Plugin for FrameCountSubscriberPluginManual {
    fn build(&self, app: &mut App) {
        init(app);
    }
}
