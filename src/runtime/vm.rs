use super::{MemoryManager, GcHeap};

pub struct VirtualMachine {
    pub memory: MemoryManager,
    pub gc: GcHeap,
    pub stack: Vec<Value>,
    pub ip: usize, // Instruction pointer
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            memory: MemoryManager::new(),
            gc: GcHeap::new(),
            stack: Vec::new(),
            ip: 0,
        }
    }

    pub fn execute(&mut self, bytecode: &[u8]) -> Result<(), String> {
        while self.ip < bytecode.len() {
            match bytecode[self.ip] {
                // Implementação das instruções
                _ => unimplemented!()
            }
            self.ip += 1;
        }
        Ok(())
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            stack: self.stack.clone(),
            memory: self.memory.snapshot(),
        }
    }
}