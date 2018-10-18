pub type Palindrome = u64;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
struct Digits {
    n: u64,
    base: u64,
}

impl Digits {
    fn new(n: u64, base: u64) -> Digits {
        Digits { n: n, base: base }
    }
}

impl Iterator for Digits {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            None
        } else {
            let res = self.n % self.base;
            self.n = self.n / self.base;
            Some(res)
        }
    }
}

fn is_palindrome(n: u64) -> bool {
    let digits = Digits::new(n, 10).collect::<Vec<u64>>();
    digits.iter().zip(digits.iter().rev()).all(|(a, b)| a == b)
}

pub fn get_palindrome_products(min: u64, max: u64) -> Vec<Palindrome> {
    let mut out = Vec::new();
    for i in min..max + 1 {
        for j in i..max + 1 {
            let n = i * j;
            if is_palindrome(n) {
                out.push(n);
            }
        }
    }
    out
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().min().cloned()
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().max().cloned()
}
