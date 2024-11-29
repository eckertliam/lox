use std::marker::PhantomData;


// Implementation based off of Ceronman's Loxido: https://github.com/ceronman/loxido/blob/master/src/gc.rs

pub trait GcTrace {}

#[derive(Debug, PartialEq, Eq)]
pub struct GcRef<T: GcTrace> {
    idx: usize,
    _marker: PhantomData<T>,
}

impl<T: GcTrace> Copy for GcRef<T> {}

impl<T: GcTrace> Clone for GcRef<T> {
    fn clone(&self) -> Self {
        *self
    }
}
