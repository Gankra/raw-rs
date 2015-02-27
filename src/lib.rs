// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(core)]
#![feature(alloc)]

//! Utilities for working with the raw representations of core Rust data types.
//!
//! This library provides convenience methods and types for doing `unsafe` work in a more
//! ergonomic way. It provides extension traits for the raw pointer types `*const T` and `*mut T`,
//! as well as the raw slice types `*const [T]` and `*mut [T]`. It also provides a wrapper for
//! slices that has all of its operations unchecked. Functionality is separated into modules so
//! that they can opted into individually using a glob import.
//!
//! For the most part, the methods provided are conveniences for those found in the `std::ptr` API,
//! which can be a bit awkward. Method names reflect a proposed set of names for `std::ptr`
//! stabilization, and not the actual names. Should `std::ptr` be stabilized, these may be renamed.
//!
//! Note also that an *ideal* implementation of this library provides many of the offered methods
//! as unsafe *operators*. Unfortunately, this is not currently possible in Rust as of this
//! writing. We instead settle for named methods that mirror these operators.

pub mod rawslice;
pub mod rawptr;
pub mod uncheckedslice;
pub mod alloc;
