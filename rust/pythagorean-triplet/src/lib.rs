pub fn find() -> Option<i32> {
    // 200, 375, 425
    let target = 1000;
    for a in 3..target {
        for b in a..target {
            let c = target - a - b;
            if a * a + b * b == c * c {
                return Some(a * b * c);
            }
        }
    }
    None
}
