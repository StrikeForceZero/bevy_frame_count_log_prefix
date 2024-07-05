use bevy::prelude::*;

use crate::formatter::FrameCountFormatter;

#[derive(Debug, Default, Resource)]
pub struct FrameCountSubscriberConfig {
    pub formatter: FrameCountFormatter,
}

impl FrameCountSubscriberConfig {
    pub fn get_frame_count_prefix_formatter(&self) -> &FrameCountFormatter {
        &self.formatter
    }
}
