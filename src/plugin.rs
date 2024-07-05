use bevy::app::{App, First, Plugin};
use bevy::log::LogPlugin;

use crate::cache_system::cache_frame_count;
use crate::config::FrameCountLogPrefixConfig;
use crate::subscriber_layer::add_log_plugin_with_custom_layer;

fn init(app: &mut App) {
    app.add_systems(First, cache_frame_count)
        .init_resource::<FrameCountLogPrefixConfig>();
}

pub struct FrameCountLogPrefixPlugin;

impl Plugin for FrameCountLogPrefixPlugin {
    fn build(&self, app: &mut App) {
        if !app.get_added_plugins::<LogPlugin>().is_empty() {
            panic!("LogPlugin already loaded, please disable with `.add_plugins(DefaultPlugins.build().disable::<LogPlugin>())` before loading FrameCountLogPrefixPlugin, \
             or add the layer manually `LogPlugin {{ custom_layer: |app| Some(Box::new(vec![/* ..other layers here, */ bevy_frame_count_log_prefix::subscriber_layer::frame_count_layer(app)])) }}`");
        }
        if !app
            .get_added_plugins::<FrameCountLogPrefixPlugin>()
            .is_empty()
        {
            panic!("FrameCountLogPrefixPlugin already loaded");
        }
        init(app);
        add_log_plugin_with_custom_layer(app);
    }
}

pub struct FrameCountLogPrefixManualPlugin;

impl Plugin for FrameCountLogPrefixManualPlugin {
    fn build(&self, app: &mut App) {
        init(app);
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::*;

    #[test]
    #[should_panic]
    fn log_plugin_already_added() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(FrameCountLogPrefixPlugin)
            .run();
    }

    #[test]
    #[should_panic]
    fn this_plugin_already_added() {
        App::new()
            .add_plugins(FrameCountLogPrefixPlugin)
            .add_plugins(FrameCountLogPrefixPlugin)
            .run();
    }
}
