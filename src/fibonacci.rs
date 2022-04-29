use std::iter::Iterator;

pub struct Fibonacci {
    x: (usize, usize)
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci{x: (0, 1)}
    }
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.x = (self.x.1, self.x.0 + self.x.1);
        Some(self.x.0)
    }
}