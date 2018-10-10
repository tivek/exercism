//#![feature(try_reserve)]
extern crate itertools;

use itertools::Itertools;

pub fn encode(source: &str) -> String {
    let groups = source.chars().group_by(|&c| c);
    let res = groups
        .into_iter()
        .map(|(c, g)| {
            let l = g.count();
            if l > 1 {
                format!("{}{}", l, c)
            } else {
                c.to_string()
            }
        }).collect::<String>();
    println!("Encoded {:?} to {:?}", source, res);
    res
}

pub fn decode(source: &str) -> String {
    let mut res = String::new();
    let mut n: usize = 0;
    for c in source.chars() {
        match c.to_digit(10) {
            Some(d) => n = n * 10 + d as usize,
            None => {
                //res.try_reserve(n);
                if n == 0 {
                    res.push(c);
                } else {
                    for _ in 0..n {
                        res.push(c);
                    }
                    n = 0;
                }
            }
        }
    }
    println!("Decoded {:?} to {:?}", source, res);
    res
}
