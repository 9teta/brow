use std::sync::Arc;

use chrono::{DateTime, FixedOffset, Utc};
use once_cell::sync::Lazy;

const PRINT_CHRONO_FORMAT_STRING: &str = "%Y-%m-%d %H:%M:%S";
const FILESYS_CHRONO_FORMAT_STRING: &str = "%Y_%m_%d_%H_%M_%S";
pub const FULL_CHRONO_FORMAT_STRING: &str = "%Y-%m-%d %H:%M:%S%.3f %:::z";
const UTC_PLUS_2: Lazy<FixedOffset> = Lazy::new(|| FixedOffset::east_opt(2 * 3600).unwrap());



pub fn now() -> DateTime<FixedOffset> {
    let utc: DateTime<Utc> = Utc::now();
    utc.with_timezone( &UTC_PLUS_2 )
}

pub fn now_pr() -> String {
    let now = now();
    now.format(PRINT_CHRONO_FORMAT_STRING).to_string()
}

pub fn now_fs() -> String {
    let now = now();
    now.format(FILESYS_CHRONO_FORMAT_STRING).to_string()
}

pub fn now_full_pr() -> String {
    let now = now();
    now.format(FULL_CHRONO_FORMAT_STRING).to_string()
}