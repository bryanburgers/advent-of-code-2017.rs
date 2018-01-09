pub struct Generator {
    cur: u64,
    factor: u64,
    div: u64,
}

impl Generator {
    pub fn new_a(init: u64) -> Generator {
        Generator { cur: init, factor: 16807, div: 2147483647 }
    }

    pub fn new_b(init: u64) -> Generator {
        Generator { cur: init, factor: 48271, div: 2147483647 }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let prod = self.cur * self.factor;
        let div = prod % self.div;
        self.cur = div;
        
        Some(div)
    }
}
