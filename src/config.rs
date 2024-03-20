use crate::formatter::FrameCountPrefixFormatter;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, Resource)]
pub struct FrameCountSubscriberConfig {
    frame_count_prefix_formatter: Option<FrameCountPrefixFormatter>,
}

impl FrameCountSubscriberConfig {
    pub fn get_frame_count_prefix_formatter(&self) -> Option<FrameCountPrefixFormatter> {
        self.frame_count_prefix_formatter
    }
}
