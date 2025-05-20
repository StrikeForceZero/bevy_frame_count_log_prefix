use bevy_ecs::prelude::Resource;

use crate::formatter::FrameCountFormatter;

#[derive(Debug, Default, Resource)]
pub struct FrameCountLogPrefixConfig {
    pub formatter: FrameCountFormatter,
}
