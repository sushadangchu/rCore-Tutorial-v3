#![no_std]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(const_in_array_repeat_expressions)]

extern crate alloc;

#[macro_use]
extern crate sbi;

#[macro_use]
mod syscall;
mod trap;
mod task;
mod fs;

pub use trap::{trap_init, enable_timer_interrupt};
pub use fs::list_apps;
pub use task::{add_initproc, run_tasks};