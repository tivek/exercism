pub fn brackets_match(open: char, closed: char) -> bool {
    (open == '(' && closed == ')')
        || (open == '[' && closed == ']')
        || (open == '{' && closed == '}')
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::<char>::new();
    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => match stack.pop() {
                None => return false,
                Some(p) => if brackets_match(p, c) {
                    continue;
                } else {
                    return false;
                },
            },
            _ => continue,
        }
    }

    stack.is_empty()
}
