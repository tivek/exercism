/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut x = Vec::<u8>::new();
    x.reserve(10);

    let mut count: usize = 0;
    for c in isbn.chars() {
        if count < 9 {
            if c.is_ascii_digit() {
                x.push(c.to_digit(10).unwrap() as u8);
                count += 1;
            }
        } else if count == 9 {
            if c.is_ascii_digit() {
                x.push(c.to_digit(10).unwrap() as u8);
                count += 1;
            } else if c == 'X' {
                x.push(10);
                count += 1;
            }
        } else {
            return false;
        }
    }
    count == 10
        && x.iter()
            .enumerate()
            .map(|(i, &n)| (10u64 - i as u64) * n as u64)
            .sum::<u64>()
            % 11
            == 0
}
