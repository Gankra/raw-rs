use super::{plain, oom};

/// Allocates and returns a ptr to memory to store a single element of type T. Handles zero-sized
/// types automatically by returning the non-null EMPTY ptr.
///
/// # Aborts
///
/// Aborts on OOM
#[inline]
pub unsafe fn alloc<T>() -> *mut T {
    let ptr = plain::alloc::<T>();
    if ptr.is_null() { oom() }
    ptr
}

/// Allocates and returns a ptr to memory to store a `len` elements of type T. Handles zero-sized
/// types automatically by returning the EMPTY ptr.
///
/// # Undefined Behaviour
///
/// * `len` must not be 0.
///
/// # Aborts
///
/// Aborts on OOM
#[inline]
pub unsafe fn alloc_array<T>(len: usize) -> *mut T {
    let ptr = plain::alloc_array::<T>(len);
    if ptr.is_null() { oom() }
    ptr
}

/// Resizes the allocation referenced by `ptr` to fit `len` elements of type T. Handles zero-sized
/// types automatically by returning the given ptr. `old_len` must be then `len` provided to the
/// call to `alloc_array` or `realloc_array` that created `ptr`.
///
/// # Undefined Behaviour
///
/// * `len` must not be 0.
///
/// # Aborts
///
/// Aborts on OOM
#[inline]
pub unsafe fn realloc_array<T>(ptr: *mut T, old_len: usize, len: usize) -> *mut T {
    let ptr = plain::realloc_array(ptr, old_len, len);
    if ptr.is_null() { oom() }
    ptr
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
pub unsafe fn try_grow_inplace<T>(ptr: *mut T, old_len: usize, len: usize) -> Result<(), ()> {
    plain::try_grow_inplace(ptr, old_len, len)
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
pub unsafe fn try_shrink_inplace<T>(ptr: *mut T, old_len: usize, len: usize) -> Result<(), ()> {
    plain::try_shrink_inplace(ptr, old_len, len)
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
    plain::dealloc(ptr);
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
pub unsafe fn dealloc_array<T>(ptr: *mut T, len: usize) {
    plain::dealloc_array(ptr, len);
}
