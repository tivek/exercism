pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return Vec::new();
    }

    let upper_bound: usize = upper_bound as usize;
    let mut is_prime = vec![true; upper_bound + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut i: usize = 2;
    while i * i <= upper_bound {
        if is_prime[i] {
            let mut j: usize = i * i;
            while j <= upper_bound {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    is_prime
        .iter()
        .enumerate()
        .filter(|(_, p)| **p)
        .map(|(n, _)| n as u64)
        .collect()
}
