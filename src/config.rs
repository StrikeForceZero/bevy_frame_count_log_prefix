use bevy::prelude::*;

use crate::formatter::FrameCountFormatter;

#[derive(Debug, Default, Resource)]
pub struct FrameCountLogPrefixConfig {
    pub formatter: FrameCountFormatter,
}

impl FrameCountLogPrefixConfig {
    pub fn get_frame_count_prefix_formatter(&self) -> &FrameCountFormatter {
        &self.formatter
    }
}
