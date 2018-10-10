extern crate rand;

use rand::Rng;

fn modular_exponentation(b: u64, e: u64, m: u64) -> u64 {
    let mut c: u64 = 1;
    for _f in 0..e {
        c = (b * c) % m;
    }
    c
}

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentation(b_pub, a, p)
}
