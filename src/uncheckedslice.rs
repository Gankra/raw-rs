use rawslice::{RawSlice, RawMutSlice, SliceRawExt};

/// A wrapper for a slice that provides unchecked versions of the standard operations.
pub struct UncheckedSlice<'a, T: 'a > {
    slice: &'a [T],
}

/// A wrapper for a mutable slice that provides unchecked versions of the standard operations.
pub struct UncheckedMutSlice<'a, T: 'a> {
    slice: &'a mut [T],
}


pub trait SliceUncheckedExt<T> for Sized? {
    /// Gets a version of the slice where all operations aren't bounds checked.
    fn unchecked<'a>(&'a self) -> UncheckedSlice<'a, T>;

    /// Gets a mutable version of the slice where all operations aren't bounds checked.
    fn unchecked_mut<'a>(&'a mut self) -> UncheckedMutSlice<'a, T>;
}

impl<T> SliceUncheckedExt<T> for [T] {
    fn unchecked<'a>(&'a self) -> UncheckedSlice<'a, T> {
        UncheckedSlice::new(self)
    }

    fn unchecked_mut<'a>(&'a mut self) -> UncheckedMutSlice<'a, T> {
        UncheckedMutSlice::new(self)
    }
}



impl<'a, T> UncheckedSlice<'a, T> {
    /// Makes a new unchecked slice from a slice.
    pub fn new(slice: &'a [T]) -> UncheckedSlice<'a, T> {
        UncheckedSlice{ slice: slice }
    }

    /// Gets the length of the slice.
    pub fn len(&self) -> uint {
        self.slice.len()
    }

    /// Converts the unchecked slice back into a checked one.
    pub fn as_slice(self) -> &'a [T] {
        self.slice
    }

    /// Gets a subslice of this one.
    pub unsafe fn slice<'b>(&'b self, from: uint, to: uint) -> UncheckedSlice<'b, T> {
        UncheckedSlice::new(self.slice.as_raw().slice(from, to).as_slice())
    }

    /// Gets a subslice from the given index to its end.
    pub unsafe fn slice_from<'b>(&'b self, from: uint) -> UncheckedSlice<'b, T> {
        self.slice(from, self.len())
    }

    /// Gets a subslice from 0 to the given index.
    pub unsafe fn slice_to<'b>(&'b self, to: uint) -> UncheckedSlice<'b, T> {
        self.slice(0, to)
    }

    /// Splits the given slice into two disjoint slices at the given index.
    pub unsafe fn split_at<'b>(&'b self, at: uint) -> (UncheckedSlice<'b, T>, UncheckedSlice<'b, T>) {
        (self.slice_to(at), self.slice_from(at))
    }

    /// Gets the value at the given index.
    pub unsafe fn get(&self, index: uint) ->  &T {
        self.slice.as_raw().get(index)
    }
}

impl<'a, T> UncheckedMutSlice<'a, T> {
    /// Makes a new unchecked slice from a slice.
    pub fn new(slice: &'a mut [T]) -> UncheckedMutSlice<'a, T> {
        UncheckedMutSlice{ slice: slice }
    }

    /// Gets the length of the slice.
    pub fn len(&self) -> uint {
        self.slice.len()
    }

    /// Converts the unchecked slice back into a checked one.
    pub fn as_slice(self) -> &'a [T] {
        self.slice
    }

    /// Converts the unchecked slice back into a mutable checked one.
    pub fn as_mut_slice(self) -> &'a mut [T] {
        self.slice
    }

    /// Gets a subslice of this one.
    pub unsafe fn slice<'b>(&'b mut self, from: uint, to: uint) -> UncheckedMutSlice<'b, T> {
        UncheckedMutSlice::new(self.slice.as_mut_raw().slice(from, to).as_mut_slice())
    }

    /// Gets a subslice from the given index to its end.
    pub unsafe fn slice_from<'b>(&'b mut self, from: uint) -> UncheckedMutSlice<'b, T> {
        let len = self.len();
        self.slice(from, len)
    }

    /// Gets a subslice from 0 to the given index.
    pub unsafe fn slice_to<'b>(&'b mut self, to: uint) -> UncheckedMutSlice<'b, T> {
        self.slice(0, to)
    }

    /// Splits the given slice into two disjoint slices at the given index.
    pub unsafe fn split_at<'b>(&'b mut self, at: uint) ->
            (UncheckedMutSlice<'b, T>, UncheckedMutSlice<'b, T>) {
        let raw = self.slice.as_mut_raw();
        (raw.slice_to(at).as_mut_slice().unchecked_mut(),
        raw.slice_from(at).as_mut_slice().unchecked_mut())
    }

    /// Gets the value at the given index.
    pub unsafe fn get(&mut self, index: uint) ->  &T {
        self.slice.as_mut_raw().get_mut(index)
    }

    /// Gets the value at the given index mutably.
    pub unsafe fn get_mut(&mut self, index: uint) ->  &mut T {
        self.slice.as_mut_raw().get_mut(index)
    }
}