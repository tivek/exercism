extern crate itertools;

use itertools::Itertools;

pub fn build_proverb(list: Vec<&str>) -> String {
    if list.is_empty() {
        return String::new();
    }

    let i = list.iter();
    let i2 = i.clone().skip(1);
    i.zip(i2)
        .map(|(a, b)| format!("For want of a {} the {} was lost.", a, b))
        .chain(std::iter::once(format!(
            "And all for the want of a {}.",
            list[0]
        ))).join("\n")
}
