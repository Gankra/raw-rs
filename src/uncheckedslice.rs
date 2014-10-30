use rawslice::{RawSlice, RawMutSlice, SliceRawExt};

pub struct UncheckedSlice<'a, T: 'a > {
    slice: &'a [T],
}

pub struct UncheckedMutSlice<'a, T: 'a> {
    slice: &'a mut [T],
}


pub trait SliceUncheckedExt<T> for Sized? {
    /// Gets a version of the slice where all operations aren't bounds checked
    fn unchecked<'a>(&'a self) -> UncheckedSlice<'a, T>;

    /// Gets a mutable version of the slice where all operations aren't bounds checked
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
    fn new(slice: &'a [T]) -> UncheckedSlice<'a, T> {
        UncheckedSlice{ slice: slice }
    }

    fn len(&self) -> uint {
        self.slice.len()
    }

    fn as_slice(self) -> &'a [T] {
        self.slice
    }

    unsafe fn slice(&self, from: uint, to: uint) -> UncheckedSlice<'a, T> {
        UncheckedSlice::new(self.slice.as_raw().slice(from, to).as_slice())
    }

    unsafe fn slice_from(&self, from: uint) -> UncheckedSlice<'a, T> {
        self.slice(from, self.len())
    }

    unsafe fn slice_to(&self, to: uint) -> UncheckedSlice<'a, T> {
        self.slice(0, to)
    }

    unsafe fn split_at(&self, at: uint) -> (UncheckedSlice<'a, T>, UncheckedSlice<'a, T>) {
        (self.slice_to(at), self.slice_from(at))
    }

    unsafe fn get(&self, index: uint) ->  &'a T {
        self.slice.as_raw().get(index)
    }
}

impl<'a, T> UncheckedMutSlice<'a, T> {
    fn new(slice: &'a mut [T]) -> UncheckedMutSlice<'a, T> {
        UncheckedMutSlice{ slice: slice }
    }

    fn len(&self) -> uint {
        self.slice.len()
    }

    fn as_slice(self) -> &'a [T] {
        self.slice
    }

    fn as_mut_slice(self) -> &'a mut [T] {
        self.slice
    }

    unsafe fn slice(&mut self, from: uint, to: uint) -> UncheckedMutSlice<'a, T> {
        UncheckedMutSlice::new(self.slice.as_mut_raw().slice(from, to).as_mut_slice())
    }

    unsafe fn slice_from(&mut self, from: uint) -> UncheckedMutSlice<'a, T> {
        let len = self.len();
        self.slice(from, len)
    }

    unsafe fn slice_to(&mut self, to: uint) -> UncheckedMutSlice<'a, T> {
        self.slice(0, to)
    }

    unsafe fn split_at(&mut self, at: uint) ->
            (UncheckedMutSlice<'a, T>, UncheckedMutSlice<'a, T>) {
        let raw = self.slice.as_mut_raw();
        (raw.slice_to(at).as_mut_slice().unchecked_mut(),
        raw.slice_from(at).as_mut_slice().unchecked_mut())
    }

    unsafe fn get(&mut self, index: uint) ->  &'a T {
        self.slice.as_mut_raw().get_mut(index)
    }

    unsafe fn get_mut(&mut self, index: uint) ->  &'a mut T {
        self.slice.as_mut_raw().get_mut(index)
    }
}