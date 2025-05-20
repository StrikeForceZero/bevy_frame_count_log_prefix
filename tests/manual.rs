use std::fmt::Formatter;
use std::sync::RwLock;

use bevy::ecs::system::RunSystemOnce;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_frame_count_log_prefix::prelude::*;

#[test]
fn main() {
    static WAS_WRITE_CALLED: RwLock<bool> = RwLock::new(false);

    struct TestFormatter;

    impl FormatFrameCount for TestFormatter {
        fn debug_name(&self) -> &'static str {
            "TestFormatter"
        }
        fn write(
            &self,
            f: &mut Formatter<'_>,
            frame_count: u32,
            #[cfg(feature = "fixed_update")] fixed_update_count: u32,
        ) -> std::fmt::Result {
            *WAS_WRITE_CALLED.write().unwrap() = true;
            #[cfg(not(feature = "fixed_update"))]
            {
                write!(f, "{frame_count} ")
            }
            #[cfg(feature = "fixed_update")]
            {
                write!(f, "{frame_count} {fixed_update_count} ")
            }
        }
    }

    let mut app = App::new();
    app.insert_resource(FrameCountLogPrefixConfig {
        formatter: FrameCountFormatter::new(TestFormatter),
    })
        .add_plugins(FrameCountLogPrefixManualPlugin)
        .add_plugins(DefaultPlugins.build().set(LogPlugin {
            custom_layer: |app| Some(Box::new(vec![frame_count_layer(app)])),
            ..default()
        }));
    // using error! to be captured by env filter
    let _ = app.world_mut().run_system_once(|| error!("test"));
    assert!(*WAS_WRITE_CALLED.read().unwrap(), "write was never called");
}
