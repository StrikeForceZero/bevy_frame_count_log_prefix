use bevy::prelude::*;

use crate::formatter::FrameCountFormatter;

#[derive(Debug, Default, Resource)]
pub struct FrameCountLogPrefixConfig {
    pub formatter: FrameCountFormatter,
}
