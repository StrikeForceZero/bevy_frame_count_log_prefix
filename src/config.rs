use bevy::prelude::*;

use crate::formatter::FrameCounterPrefixFormatter;

#[derive(Debug, Default, Resource)]
pub struct FrameCountSubscriberConfig {
    pub frame_count_prefix_formatter: FrameCounterPrefixFormatter,
}

impl FrameCountSubscriberConfig {
    pub fn get_frame_count_prefix_formatter(&self) -> &FrameCounterPrefixFormatter {
        &self.frame_count_prefix_formatter
    }
}
