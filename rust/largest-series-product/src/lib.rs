#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    let mut out = Vec::<u64>::new();
    for c in string_digits.chars() {
        match c.to_digit(10) {
            Some(n) => out.push(n as u64),
            None => return Err(Error::InvalidDigit(c)),
        }
    }

    if span == 0 {
        return Ok(1);
    }

    out.as_slice()
        .windows(span)
        .map(|w| w.iter().product())
        .max()
        .ok_or(Error::SpanTooLong)
}
