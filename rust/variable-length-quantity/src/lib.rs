#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut out = Vec::new();
    for mut v in values.iter().rev().cloned() {
        out.push((v & 127) as u8);
        v >>= 7;
        while v != 0 {
            out.push((v & 127 | 128) as u8);
            v >>= 7;
        }
    }
    out.iter().rev().cloned().collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut out = Vec::new();
    let mut v = 0u32;
    for &b in bytes {
        if v & 0xfe000000 != 0 {
            return Err(Error::Overflow);
        }
        v <<= 7;
        v |= (b & 127) as u32;
        if b & 128 == 0 {
            out.push(v);
            v = 0;
        }
    }
    if out.len() == 0 || v != 0 {
        return Err(Error::IncompleteNumber);
    }
    Ok(out)
}
