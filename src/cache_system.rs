#[cfg(feature = "fixed_update")]
use crate::fixed_update_count::FixedUpdateCount;
use bevy_diagnostic::FrameCount;
use bevy_ecs::prelude::Res;

pub(crate) fn cache_frame_count(frame_count: Res<FrameCount>) {
    crate::statics::set_frame_count(frame_count.0);
}

#[cfg(feature = "fixed_update")]
pub(crate) fn cache_fixed_update_count(fixed_update: Res<FixedUpdateCount>) {
    crate::statics::set_fixed_update_count(fixed_update.0);
}
