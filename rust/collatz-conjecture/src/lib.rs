pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut count = 0;
    let mut n = n;
    while n != 1 {
        count += 1;
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
    }
    Some(count)
}
