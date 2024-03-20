use std::sync::atomic::{AtomicU32, Ordering};

static FRAME_COUNTER: AtomicU32 = AtomicU32::new(0);

pub(crate) fn set_frame_count(count: u32) {
    FRAME_COUNTER.store(count, Ordering::Relaxed);
}

pub(crate) fn get_frame_count() -> u32 {
    FRAME_COUNTER.load(Ordering::SeqCst)
}
