use std::{cell::Cell, fmt, hash::Hash, marker::PhantomData};

/// A trait for types that can be used as indices into a vector.
pub trait Idx: 'static + Copy + Eq + Hash + fmt::Debug {
    fn index(&self) -> usize;

    fn new(index: usize) -> Self;
}

/// A wrapper around a `usize` that implements `Idx`.
#[derive(Clone, Debug)]
pub struct Idxr<T> {
    next: Cell<usize>,
    _phantom: PhantomData<T>,
}

impl<T: Idx> Idxr<T> {
    pub fn new() -> Idxr<T> {
        Idxr {
            next: Cell::new(0),
            _phantom: PhantomData::default(),
        }
    }

    pub fn from(start: usize) -> Idxr<T> {
        Idxr {
            next: Cell::new(start),
            _phantom: PhantomData::default(),
        }
    }

    pub fn next(&self) -> T {
        let next = self.next.get();
        let local_idx = T::new(next);
        self.next.set(next + 1);
        local_idx
    }
}
