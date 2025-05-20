use bevy_diagnostic::FrameCount;
use bevy_ecs::prelude::Res;

pub(crate) fn cache_frame_count(frame_count: Res<FrameCount>) {
    crate::statics::set_frame_count(frame_count.0);
}
