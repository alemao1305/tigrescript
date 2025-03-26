pub mod memory;
pub mod gc;
pub mod vm;

pub use memory::MemoryManager;
pub use gc::GcHeap;
pub use vm::VirtualMachine;