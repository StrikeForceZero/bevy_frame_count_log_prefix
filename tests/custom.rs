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
    app.add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .insert_resource(FrameCountLogPrefixConfig {
            formatter: TestFormatter.into(),
        })
        .add_plugins(FrameCountLogPrefixPlugin);
    // using error! to be captured by env filter
    let _ = app.world_mut().run_system_once(|| error!("test"));
    assert!(*WAS_WRITE_CALLED.read().unwrap(), "write was never called");
}
