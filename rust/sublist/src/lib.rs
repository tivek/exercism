#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn sublist_worker<I1, I2, T: PartialEq>(a: I1, b: I2) -> bool
where
    I1: IntoIterator<Item = T>,
    I1::IntoIter: Clone,
    I2: IntoIterator<Item = T>,
    I2::IntoIter: Clone,
{
    let a_it_first = a.into_iter();
    let mut b_it_last = b.into_iter();

    let mut a_it = a_it_first.clone();
    let mut b_it = b_it_last.clone();

    loop {
        match (a_it.next(), b_it.next()) {
            (Some(aval), Some(bval)) => if aval != bval {
                a_it = a_it_first.clone();
                b_it_last.next();
                b_it = b_it_last.clone();
            },
            (Some(_), None) => return false,
            (None, _) => return true,
        }
    }
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let f_sub_s = sublist_worker(first_list, second_list);
    let s_sub_f = sublist_worker(second_list, first_list);

    match (f_sub_s, s_sub_f) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
