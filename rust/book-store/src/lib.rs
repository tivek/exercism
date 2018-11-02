extern crate itertools;

use itertools::Itertools;

fn price_of_pick(uniques: usize) -> u32 {
    (8 * uniques * match uniques {
        1 => 100,
        2 => 95,
        3 => 90,
        4 => 80,
        _ => 75,
    }) as u32
}

fn lowest_price_recursive(mut cart: Vec<usize>) -> u32 {
    cart.retain(|&cnt| cnt > 0);

    if cart.iter().all(|&i| i == 1) {
        return price_of_pick(cart.len());
    }

    if cart.len() == 1 {
        return price_of_pick(1) * cart[0] as u32;
    }

    let picks_it = (2..(cart.len() + 1)).map(|i| (0..cart.len()).combinations(i));
    std::iter::Iterator::flatten(picks_it)
        .map(|selection| {
            price_of_pick(selection.len()) + {
                let mut rest = cart.clone();
                for b in selection {
                    rest[b] -= 1;
                }
                lowest_price_recursive(rest)
            }
        }).min()
        .unwrap()
}

pub fn lowest_price(books: &[u32]) -> u32 {
    if let Some(&num_books) = books.iter().max() {
        let mut cart = vec![0usize; num_books as usize];
        for &b in books {
            cart[b as usize - 1] += 1;
        }
        lowest_price_recursive(cart)
    } else {
        0
    }
}
