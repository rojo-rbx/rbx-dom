use std::sync::atomic::{AtomicU64, Ordering};

use once_cell::sync::OnceCell;

static LAST_ID: OnceCell<AtomicU64> = OnceCell::new();

pub fn get() -> u64 {
    LAST_ID
        .get_or_init(|| AtomicU64::new(1))
        .fetch_add(1, Ordering::SeqCst)
}
