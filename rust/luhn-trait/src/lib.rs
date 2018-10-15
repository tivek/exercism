pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let code = self.to_string();
        if !code.chars().all(|c: char| c.is_ascii_digit() || c == ' ') {
            return false;
        }

        let mut s = 0;
        let mut i = 0;
        for mut n in code.chars().rev().filter_map(|c| c.to_digit(10)) {
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
