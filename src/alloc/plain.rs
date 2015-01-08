use std::rt::heap::{self, usable_size, EMPTY};
use std::mem::{size_of, min_align_of};
use std::num::Int;
use std::uint;

/// Allocates and returns a ptr to memory to store a single element of type T. Handles zero-sized
/// types automatically by returning the non-null EMPTY ptr. Returns `null` on OOM.
#[inline]
pub unsafe fn alloc<T>() -> *mut T {
    let size = size_of::<T>();
    if size == 0 {
        EMPTY as *mut T
    } else {
        heap::allocate(size, min_align_of::<T>()) as *mut T
    }
}

/// Allocates and returns a ptr to memory to store a `len` elements of type T. Handles zero-sized
/// types automatically by returning the EMPTY ptr. Returns `null` on OOM.
///
/// # Undefined Behaviour
///
/// * `len` must not be 0.
#[inline]
pub unsafe fn alloc_array<T>(len: uint) -> *mut T {
    debug_assert!(len != 0, "0 len passed to alloc_array");
    let size = size_of::<T>();
    if size == 0 {
        EMPTY as *mut T
    } else {
        let desired_size = size.checked_mul(len).unwrap_or(uint::MAX);
        heap::allocate(desired_size, min_align_of::<T>()) as *mut T
    }
}

/// Resizes the allocation referenced by `ptr` to fit `len` elements of type T. Handles zero-sized
/// types automatically by returning the given ptr. `old_len` must be then `len` provided to the
/// call to `alloc_array` or `realloc_array` that created `ptr`. Returns `null` on OOM.
///
/// # Undefined Behaviour
///
/// * `len` must not be 0.
#[inline]
pub unsafe fn realloc_array<T>(ptr: *mut T, old_len: uint, len: uint) -> *mut T {
    debug_assert!(len != 0, "0 len passed to realloc_array");
    let size = size_of::<T>();
    if size == 0 {
        ptr
    } else {
        let desired_size = size.checked_mul(len).unwrap_or(uint::MAX);
        let align = min_align_of::<T>();
        // No need to check old_size * len, must have been checked when the ptr was made, or
        // else UB anyway.
        heap::reallocate(ptr as *mut u8, size * old_len, desired_size, align) as *mut T
    }
}

/// Tries to grow the allocation referenced by `ptr` in-place to fit `len` elements of type `T`.
/// If successful, yields `Ok`. If unsuccessful, yields `Err`, and the allocation is unchanged.
/// Handles zero-sized types by always returning `Ok`.
///
/// # Undefined Behaviour
///
/// * `old_len` must be the `len` provided to the last successful allocator call that created or
/// changed `ptr`.
/// * `len` must not be 0.
/// * `len` must not be smaller than `old_len`.
#[inline]
pub unsafe fn try_grow_inplace<T>(ptr: *mut T, old_len: uint, len: uint) -> Result<(), ()> {
    debug_assert!(len >= old_len, "new len smaller than old_len in try_grow_inplace");
    let size = size_of::<T>();
    let align = min_align_of::<T>();
    if size == 0 {
        Ok(())
    } else {
        let desired_size = size.checked_mul(len).unwrap_or(uint::MAX);
        // No need to check size * old_len, must have been checked when the ptr was made, or
        // else UB anyway.
        let result_size = heap::reallocate_inplace(ptr as *mut u8, size * old_len,
                                                    desired_size, align);
        if result_size >= desired_size {
            Ok(())
        } else {
            Err(())
        }
    }
}

/// Tries to shrink the allocation referenced by `ptr` in-place to fit `len` elements of type `T`.
/// If successful, yields `Ok`. If unsuccessful, yields `Err`, and the allocation is unchanged.
/// Handles zero-sized types by always returning `Ok`.
///
/// # Undefined Behaviour
///
/// * `old_len` must be the `len` provided to the last successful allocator call that created or
/// changed `ptr`.
/// * `len` must not be 0.
/// * `len` must not be larger than `old_len`.
#[inline]
pub unsafe fn try_shrink_inplace<T>(ptr: *mut T, old_len: uint, len: uint) -> Result<(), ()> {
    debug_assert!(len != 0, "0 len passed to try_shrink_inplace");
    debug_assert!(len <= old_len, "new len bigger than old_len in try_grow_inplace");
    let size = size_of::<T>();
    let align = min_align_of::<T>();
    if size == 0 {
        Ok(())
    } else {
        // No need to check either mul, size * len <= size * old_len, and size * old_len must have
        // been checked when the ptr was made, or else UB anyway.
        let desired_size = size * len;
        let result_size = heap::reallocate_inplace(ptr as *mut u8, size * old_len,
                                                    desired_size, align);
        if result_size == usable_size(desired_size, align) {
            Ok(())
        } else {
            Err(())
        }
    }
}


/// Deallocates the memory referenced by `ptr`, assuming it was allocated with `alloc`.
/// Handles zero-sized types automatically by doing nothing.
///
/// # Undefined Behaviour
///
/// * The `ptr` must have been allocated by this API's `alloc` method.
/// * The `ptr` must not have been previously deallocated.
#[inline]
pub unsafe fn dealloc<T>(ptr: *mut T) {
    let size = size_of::<T>();
    if size == 0 {
        // Do nothing
    } else {
        heap::deallocate(ptr as *mut u8, size, min_align_of::<T>());
    }
}

/// Deallocates the memory referenced by `ptr`, assuming it was allocated with `alloc_array` or
/// `realloc_array`. Handles zero-sized types automatically by doing nothing.
///
/// # Undefined Behaviour
///
/// * The `ptr` must have been allocated by this API's `alloc_array` or `realloc_array` methods.
/// * The `ptr` must not have been previously deallocated.
/// * `len` must be the `len` provided to the last successful allocator call that created or
/// changed `ptr`.
#[inline]
pub unsafe fn dealloc_array<T>(ptr: *mut T, len: uint) {
    let size = size_of::<T>();
    if size == 0 {
        // Do nothing
    } else {
        // No need to check size * len, must have been checked when the ptr was made, or
        // else UB anyway.
        heap::deallocate(ptr as *mut u8, size * len, min_align_of::<T>());
    }
}
