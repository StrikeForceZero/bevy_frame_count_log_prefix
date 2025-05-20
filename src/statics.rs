use std::sync::atomic::{AtomicU32, Ordering};

static FRAME_COUNTER: AtomicU32 = AtomicU32::new(0);

pub(crate) fn set_frame_count(count: u32) {
    FRAME_COUNTER.store(count, Ordering::Relaxed);
}

pub(crate) fn get_frame_count() -> u32 {
    FRAME_COUNTER.load(Ordering::SeqCst)
}

#[cfg(feature = "fixed_update")]
static FIXED_UPDATE_COUNTER: AtomicU32 = AtomicU32::new(0);

#[cfg(feature = "fixed_update")]
pub(crate) fn set_fixed_update_count(count: u32) {
    FIXED_UPDATE_COUNTER.store(count, Ordering::Relaxed);
}

#[cfg(feature = "fixed_update")]
pub(crate) fn get_fixed_update_count() -> u32 {
    FIXED_UPDATE_COUNTER.load(Ordering::SeqCst)
}
