#![no_std]

extern crate alloc;

#[macro_use]
extern crate sbi;

mod block;

pub use block::BLOCK_DEVICE;