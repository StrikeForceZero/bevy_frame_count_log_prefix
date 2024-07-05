use bevy::prelude::*;

use crate::formatter::FrameCounterPrefixFormatter;

#[derive(Debug, Default, Resource)]
pub struct FrameCountSubscriberConfig {
    pub formatter: FrameCounterPrefixFormatter,
}

impl FrameCountSubscriberConfig {
    pub fn get_frame_count_prefix_formatter(&self) -> &FrameCounterPrefixFormatter {
        &self.formatter
    }
}
