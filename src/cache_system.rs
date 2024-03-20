use bevy::core::FrameCount;
use bevy::prelude::*;

pub(crate) fn cache_frame_count(frame_count: Res<FrameCount>) {
    crate::statics::set_frame_count(frame_count.0);
}
