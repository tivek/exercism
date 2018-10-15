use std::cmp::{max, min};

fn count_around_mines(minefield: &[&str]) -> Vec<Vec<u32>> {
    let nrows = minefield.len();
    let ncols = match minefield.first() {
        Some(r) => r.len(),
        None => return Vec::new(),
    };

    let mut counts: Vec<Vec<u32>> = Vec::new();
    counts.reserve_exact(nrows);
    for _ in 0..nrows {
        counts.push(vec![0; ncols]);
    }

    for (y, row) in minefield.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '*' {
                for j in (max(1, y) - 1)..min(y + 2, nrows) {
                    for i in (max(1, x) - 1)..min(x + 2, ncols) {
                        counts[j][i] += 1;
                    }
                }
            }
        }
    }

    counts
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let counts = count_around_mines(minefield);
    minefield
        .iter()
        .zip(counts)
        .map(|(minerow, countrow)| {
            minerow
                .chars()
                .zip(countrow.iter())
                .map(|(item, &count)| match item {
                    '*' => '*',
                    _ => match count {
                        0 => ' ',
                        c => std::char::from_digit(c, 10).unwrap(),
                    },
                }).collect()
        }).collect()
}
