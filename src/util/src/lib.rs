#![no_std]
#![feature(nonzero, alloc)]

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

pub use nulltermarray::NullTermArrayIter;

mod nulltermarray;
