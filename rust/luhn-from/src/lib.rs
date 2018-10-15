pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if !self
            .code
            .chars()
            .all(|c: char| c.is_ascii_digit() || c == ' ')
        {
            return false;
        }

        let mut s = 0;
        let mut i = 0;
        for mut n in self.code.chars().rev().filter_map(|c| c.to_digit(10)) {
            if i % 2 == 1 {
                n *= 2;
                if n > 9 {
                    n -= 9
                }
            }
            s += n;
            i += 1;
        }

        i > 1 && s % 10 == 0
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn {
            code: input.to_string(),
        }
    }
}
