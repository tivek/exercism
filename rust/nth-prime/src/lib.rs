fn erathostenes(bound: u32) -> Vec<u32> {
    let mut primes: Vec<bool> = (0..bound + 1).map(|num| num == 2 || num % 2 == 1).collect();
    let mut num: u32 = 3;
    while num * num <= bound {
        let mut j = num * num;
        while j <= bound {
            primes[j as usize] = false;
            j += num;
        }
        num += 2;
    }
    primes
        .into_iter()
        .enumerate()
        .skip(2)
        .filter_map(|(i, p)| if p { Some(i as u32) } else { None })
        .collect::<Vec<u32>>()
}

pub fn nth(n: u32) -> u32 {
    let bound: u32 = if n >= 6 {
        let lnn = (n as f32).ln();
        (n as f32 * (lnn + lnn.ln())).ceil() as u32
    } else {
        15
    };
    erathostenes(bound)[n as usize] as u32
}
