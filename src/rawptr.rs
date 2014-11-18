// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;
use std::ptr;
use std::raw::Slice;
use rawslice::{RawSlice, RawMutSlice};

/// Extension trait for convenience methods on raw pointers
pub trait RawPtrExt<T>: RawPtr<T> {
    /// Converts the pointer into a raw slice.
    fn as_raw_slice(self, len: uint) -> *const [T];

    /// Converts the pointer into a slice.
    unsafe fn as_slice<'a>(self, len: uint) -> &'a [T];

    /// Calculates the offset from a pointer by addition. The offset *must* be in-bounds of
    /// the object, or one-byte-past-the-end.  `count` is in units of T; e.g. a
    /// `count` of 3 represents a pointer offset of `3 * sizeof::<T>()` bytes.
    unsafe fn add(self, count: uint) -> Self {
        self.offset(count as int)
    }

    // Calculates the offset from a pointer by subtraction. The offset *must* be in-bounds of
    /// the object, or one-byte-past-the-end.  `count` is in units of T; e.g. a
    /// `count` of 3 represents a pointer offset of `3 * sizeof::<T>()` bytes.
    unsafe fn sub(self, count: uint) -> Self {
        self.offset(-(count as int))
    }

    /// Reads the value from `self` and returns it.
    unsafe fn read(self) -> T;
}

/// Extension trait for convenience methods on mutable raw pointers
pub trait RawMutPtrExt<T>: RawPtrExt<T> {
    /// Converts the pointer into a raw mutable slice.
    fn as_raw_mut_slice(self, len: uint) -> *mut [T];

    /// Converts the pointer into a mutable slice.
    unsafe fn as_mut_slice<'a>(self, len: uint) -> &'a mut [T];

    /// Unsafely overwrite a memory location with the given value without destroying
    /// the old value.
    ///
    /// This operation is unsafe because it does not destroy the previous value
    /// contained at the location `dst`. This could leak allocations or resources,
    /// so care must be taken to previously deallocate the value at `dst`.
    unsafe fn write(self, src: T);

    /// Sets the `count * size_of<T>()` bytes at the address of this pointer to the the given
    /// byte. Good for zeroing out memory.
    unsafe fn write_bytes(self, byte: u8, count: uint);

    /// Copies `count * size_of<T>()` many bytes from `src` to the address of this pointer,
    /// assuming that the source and destination *may* overlap.
    unsafe fn copy(self, src: *const T, count: uint);

    /// Copies `count * size_of<T>()` many bytes from `src` to the address of this pointer,
    /// assuming that the source and destination *do not* overlap.
    unsafe fn copy_nonoverlapping(self, src: *const T, count: uint);

    /// Swaps the values of `self` and `y`. Note that in contrast to `mem::swap`, `x` and `y`
    /// may point to the same address of memory. Useful for making some operations branchless.
    unsafe fn swap(self, y: *mut T);

    /// Replace the value of the pointer, returning the old value. This is simply
    /// a convenience for calling `mem::replace` with a raw pointer.
    unsafe fn replace(self, src: T) -> T;
}

impl<T> RawPtrExt<T> for *const T {
    fn as_raw_slice(self, len: uint) -> *const [T] {
        unsafe {
            mem::transmute(Slice {
                data: self,
                len: len
            })
        }
    }

    unsafe fn as_slice<'a>(self, len: uint) -> &'a [T] {
        self.as_raw_slice(len).as_slice()
    }

    unsafe fn read(self) -> T {
        ptr::read(self)
    }
}

impl<T> RawPtrExt<T> for *mut T {
    fn as_raw_slice(self, len: uint) -> *const [T] {
        (self as *const T).as_raw_slice(len)
    }

    unsafe fn as_slice<'a>(self, len: uint) -> &'a [T] {
        self.as_raw_slice(len).as_slice()
    }

    unsafe fn read(self) -> T {
        ptr::read(self as *const T)
    }
}

impl<T> RawMutPtrExt<T> for *mut T {
    fn as_raw_mut_slice(self, len: uint) -> *mut [T] {
        unsafe {
            mem::transmute(Slice {
                data: self as *const T,
                len: len
            })
        }
    }

    unsafe fn as_mut_slice<'a>(self, len: uint) -> &'a mut [T] {
        self.as_raw_mut_slice(len).as_mut_slice()
    }

    unsafe fn write(self, src: T) {
        ptr::write(self, src);
    }

    unsafe fn write_bytes(self, byte: u8, count: uint) {
        ptr::set_memory(self, byte, count);
    }

    unsafe fn copy(self, src: *const T, count: uint) {
        ptr::copy_memory(self, src, count);
    }

    unsafe fn copy_nonoverlapping(self, src: *const T, count: uint) {
        ptr::copy_nonoverlapping_memory(self, src, count);
    }

    unsafe fn swap(self, y: *mut T) {
        ptr::swap(self, y);
    }

    unsafe fn replace(self, src: T) -> T {
        ptr::replace(self, src)
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_arithmetic() {
        unsafe {
            let mut x = [1u,2,3,4];
            let y = x[].as_ptr();
            assert_eq!(*y, 1);
            assert_eq!(*y.add(2), 3);
            assert_eq!(*y.add(2).sub(1), 2);

            let y = x[mut].as_mut_ptr();
            assert_eq!(*y, 1);
            assert_eq!(*y.add(2), 3);
            assert_eq!(*y.add(2).sub(1), 2);
        }
    }

    #[test]
    fn test_read_write() {
        unsafe {
            let x = &mut 1u as *mut _;
            assert_eq!(x.read(), 1);
            x.write(2);
            assert_eq!(x.read(), 2);
            x.write_bytes(0, 1);
            assert_eq!(x.read(), 0);
        }
    }

    #[test]
    fn test_copy() {
        unsafe {
            let mut x = [1u,2,3,4];
            let y = [5u,6,7,8];
            let xptr = x[mut].as_mut_ptr();
            let yptr = y[].as_ptr();

            xptr.copy(xptr.add(1) as *const _, 2);
            assert_eq!(x[], [2,3,3,4][]);
            xptr.copy_nonoverlapping(yptr, 4);
            assert_eq!(x[], y[]);
        }
    }

    #[test]
    fn test_swap_replace() {
        unsafe {
            let x = &mut 1u as *mut _;
            let y = &mut 2;
            x.swap(y);
            assert_eq!(*x, 2);
            assert_eq!(*y, 1);


            x.replace(3);
            assert_eq!(*x, 3);
        }
    }
}