pub fn init() {
    memory::heap_allocator::init_heap();
    memory::frame_allocator::init_frame_allocator();
    memory::KERNEL_SPACE.clone().lock().activate();
}