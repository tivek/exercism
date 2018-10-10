struct LazyFactorizer {
    n: u64,
    candidate: u64,
}

impl LazyFactorizer {
    fn new(n: u64) -> LazyFactorizer {
        LazyFactorizer { n: n, candidate: 2 }
    }
}

impl Iterator for LazyFactorizer {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.n > 1 {
            while self.n % self.candidate == 0 {
                self.n /= self.candidate;
                return Some(self.candidate);
            }
            self.candidate += 1;
        }
        None
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    LazyFactorizer::new(n).collect()
}
