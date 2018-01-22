
struct Scope {
    scratch: RegSet,
}

#[derive(Clone)]
struct RegSet {
    used: usize,
    available: usize,
}

impl RegSet {
    fn new(available: usize) -> Self {
        Self { available, used: 0 }
    }

    fn alloc(&mut self) -> Option<usize> {
        if self.available == 0 {
            return None
        }

        self.used += 1;
        self.available -= 1;
        Some(self.used - 1)
    }
}
