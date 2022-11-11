// Lamport clock
use std::sync::atomic::{AtomicUsize, Ordering};

static CLOCK: AtomicUsize = AtomicUsize::new(0);

pub fn get_timestamp() -> usize {
    CLOCK.fetch_add(1, Ordering::Relaxed)
}
