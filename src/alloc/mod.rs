//! Utilities for dealing with the boilerplate of using the allocator directly. In particular,
//! correctly handles zero-sized types and checks for integer overflows in allocation requests.
//!
//! `plain` will return a `null` pointer on OOM, while `lazy` will panic. Otherwise the two
//! APIs are identical.

use std::intrinsics::abort;

pub mod lazy;
pub mod plain;

pub fn oom() -> ! {
    unsafe { abort() }
}
