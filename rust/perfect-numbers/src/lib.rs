#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    Some({
        let s: u64 = (1..num).filter(|i| num % i == 0).sum();
        if s == num {
            Classification::Perfect
        } else if s < num {
            Classification::Deficient
        } else {
            Classification::Abundant
        }
    })
}
