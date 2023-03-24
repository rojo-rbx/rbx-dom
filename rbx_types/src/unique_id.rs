use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::{
    convert::TryFrom,
    sync::atomic::{AtomicU32, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// The `UniqueId` epoch (2021-01-01 00:00:00 GMT) in terms of time since the Unix epoch
const EPOCH_AS_UNIX: u64 = 1_609_459_200;

lazy_static! {
    /// A `SystemTime` representing the `UniqueId` epoch.
    pub static ref EPOCH: SystemTime = UNIX_EPOCH - Duration::from_secs(EPOCH_AS_UNIX);
}

/// Represents an error that can occur when constructing a new `UniqueId`.
#[derive(Debug, Error)]
pub enum UniqueIdError {
    #[error("SystemTime generated a timestamp that is before the UniqueId epoch")]
    SystemPastTime,
    #[error("UniqueId timestamp is more than 2^32 - 1 seconds past epoch")]
    Overflow,
}

/// Represents a UUID with a custom epoch of midnight January 1st 2021.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UniqueId {
    index: u32,
    time: u32,
    random: i64,
}

static INDEX: AtomicU32 = AtomicU32::new(0);

impl UniqueId {
    pub fn new(index: u32, time: u32, random: i64) -> Self {
        Self {
            index,
            time,
            random,
        }
    }

    pub fn now() -> Result<Self, UniqueIdError> {
        let time = SystemTime::now()
            .duration_since(*EPOCH)
            .map_err(|_| UniqueIdError::SystemPastTime)?;

        Ok(Self {
            index: INDEX.fetch_add(1, Ordering::AcqRel),
            time: u32::try_from(time.as_secs()).map_err(|_| UniqueIdError::Overflow)?,
            // This matches Roblox's behavior, where the value is both an i64
            // but is also always positive.
            random: thread_rng().gen_range(0..i64::MAX),
        })
    }

    pub fn time(&self) -> u32 {
        self.time
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn random(&self) -> i64 {
        self.random
    }
}
