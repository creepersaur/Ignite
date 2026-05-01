pub struct RegisterAllocator {
    next: u8,
    free: Vec<u8>,
}

impl RegisterAllocator {
    pub fn new() -> Self {
        Self {
            next: 0,
            free: vec![],
        }
    }

    pub fn alloc(&mut self) -> u8 {
        self.free.pop().unwrap_or_else(|| {
			assert!(self.next < 255, "register overflow");
            let reg = self.next;
            self.next += 1;
            reg
        })
    }

    pub fn free(&mut self, reg: u8) {
        self.free.push(reg);
    }
}
