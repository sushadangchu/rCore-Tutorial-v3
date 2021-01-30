#![no_std]
#![no_main]
#![feature(global_asm)]

#[macro_use]
extern crate sbi;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");
    memory::init();
    os_core::trap_init();
    os_core::enable_timer_interrupt();
    sbi::set_next_trigger();
    os_core::list_apps();
    os_core::add_initproc();
    os_core::run_tasks();
    panic!("Unreachable in rust_main!");
}