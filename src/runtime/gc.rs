pub struct GcHeap {
    objects: Vec<GcObject>,
    marked: Vec<bool>,
}

impl GcHeap {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            marked: Vec::new(),
        }
    }

    pub fn alloc(&mut self, obj: GcObject) -> GcPointer {
        let ptr = GcPointer(self.objects.len());
        self.objects.push(obj);
        self.marked.push(false);
        ptr
    }

    pub fn collect(&mut self) {
        self.mark();
        self.sweep();
    }

    fn mark(&mut self) {
        // Implementação do mark phase
        for i in 0..self.objects.len() {
            if needs_marking(&self.objects[i]) {
                self.marked[i] = true;
            }
        }
    }

    fn sweep(&mut self) {
        let mut i = 0;
        while i < self.objects.len() {
            if !self.marked[i] {
                self.objects.remove(i);
                self.marked.remove(i);
            } else {
                self.marked[i] = false;
                i += 1;
            }
        }
    }
}