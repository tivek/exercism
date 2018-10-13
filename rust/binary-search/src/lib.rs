use std::cmp::Ordering;

pub fn find<T: Ord, A: AsRef<[T]>>(array: A, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mut l: usize = 0;
    let mut r: usize = array.len();
    while l <= r {
        let m = (l + r) / 2;
        if m >= array.len() {
            return None;
        }
        match key.cmp(&array[m]) {
            Ordering::Greater => {
                l = m + 1;
            }
            Ordering::Less => {
                if m == 0 {
                    return None;
                } else {
                    r = m - 1;
                }
            }
            Ordering::Equal => return Some(m),
        }
    }
    None
}
