mod memory_set;

pub use memory_set::{MemorySet, KERNEL_SPACE, MapPermission, kernel_token};

pub fn init() {
    memory::heap_allocator::init_heap();
    memory::frame_allocator::init_frame_allocator();
    KERNEL_SPACE.clone().lock().activate();
}