pub fn get_diamond(c: char) -> Vec<String> {
    let mut out = Vec::new();
    let n = (c as u8 - 'A' as u8) as usize;
    for (i, a) in ('A' as u8..c as u8 + 1)
        .enumerate()
        .chain(('A' as u8..c as u8).enumerate().rev())
    {
        let mut v = vec![' '; 2 * n + 1];
        v[n + i] = a as char;
        v[n - i] = a as char;
        out.push(v.into_iter().collect());
    }
    out
}
