#![no_std]
#![feature(llvm_asm)]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

extern crate bitflags;

pub mod heap_allocator;
pub mod address;
pub mod frame_allocator;
pub mod page_table;

pub use page_table::PTEFlags;
pub use address::VPNRange;
pub use address::{PhysAddr, VirtAddr, PhysPageNum, VirtPageNum, StepByOne};
pub use frame_allocator::{FrameTracker, frame_alloc, frame_dealloc,};
pub use page_table::{
    PageTable,
    PageTableEntry,
    translated_byte_buffer,
    translated_str,
    translated_refmut,
    UserBuffer,
    UserBufferIterator,
};