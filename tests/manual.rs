use std::fmt::Formatter;
use std::sync::RwLock;

use bevy::ecs::system::RunSystemOnce;
use bevy::log::LogPlugin;
use bevy::prelude::*;

use bevy_frame_count_subscriber::config::FrameCountSubscriberConfig;
use bevy_frame_count_subscriber::formatter::{FormatFrameCount, FrameCounterPrefixFormatter};
use bevy_frame_count_subscriber::plugin::FrameCountSubscriberPluginManual;
use bevy_frame_count_subscriber::subscriber_layer::frame_count_layer;

#[test]
fn main() {
    static WAS_WRITE_CALLED: RwLock<bool> = RwLock::new(false);

    struct TestFormatter;

    impl FormatFrameCount for TestFormatter {
        fn debug_name(&self) -> &'static str {
            "TestFormatter"
        }
        fn write(&self, f: &mut Formatter<'_>, frame_count: u32) -> std::fmt::Result {
            *WAS_WRITE_CALLED.write().unwrap() = true;
            write!(f, "{frame_count} ")
        }
    }

    let mut app = App::new();
    app.insert_resource(FrameCountSubscriberConfig {
        frame_count_prefix_formatter: FrameCounterPrefixFormatter::new(TestFormatter),
    })
    .add_plugins(FrameCountSubscriberPluginManual)
    .add_plugins(DefaultPlugins.build().set(LogPlugin {
        custom_layer: |app| Some(Box::new(vec![frame_count_layer(app)])),
        ..default()
    }));
    // using error! to be captured by env filter
    app.world_mut().run_system_once(|| error!("test"));
    assert!(*WAS_WRITE_CALLED.read().unwrap(), "write was never called");
}
