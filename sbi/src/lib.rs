#![no_std]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
pub mod console;
pub mod sbi;
pub mod timer;
pub mod lang_items;

pub use sbi::{console_putchar, set_timer, console_getchar, shutdown};
pub use timer::{get_time, get_time_ms, set_next_trigger};
