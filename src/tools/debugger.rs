pub struct Debugger {
    vm: VirtualMachine,
    breakpoints: HashSet<usize>,
    history: Vec<Snapshot>,
}

impl Debugger {
    pub fn new(vm: VirtualMachine) -> Self {
        Self {
            vm,
            breakpoints: HashSet::new(),
            history: Vec::new(),
        }
    }

    pub fn add_breakpoint(&mut self, line: usize) {
        self.breakpoints.insert(line);
    }

    pub fn step(&mut self) -> Result<(), String> {
        let snapshot = self.vm.snapshot();
        self.history.push(snapshot);
        self.vm.step()?;
        Ok(())
    }
}