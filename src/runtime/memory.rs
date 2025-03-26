pub struct MemoryManager {
    stack: Vec<u8>,
    heap: GcHeap,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(1024 * 1024), // 1MB stack
            heap: GcHeap::new(),
        }
    }

    pub fn alloc_stack(&mut self, size: usize) -> *mut u8 {
        let ptr = unsafe { self.stack.as_mut_ptr().add(self.stack.len()) };
        self.stack.reserve(size);
        unsafe { self.stack.set_len(self.stack.len() + size) };
        ptr
    }
}