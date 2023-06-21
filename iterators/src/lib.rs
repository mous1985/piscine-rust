#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 {
            None
        } else {
            self.v = if self.v % 2 == 0 {
                self.v / 2
            } else {
                3 * self.v + 1
            };
            Some(Collatz { v: self.v })
        }
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    let mut collatz = Collatz::new(n);
    let mut steps = 0;
    while let Some(_) = collatz.next() {
        steps += 1;
    }
    steps
}