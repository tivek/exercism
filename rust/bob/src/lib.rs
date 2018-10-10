pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let yells = message == message.to_uppercase() && message != message.to_lowercase();
    let is_question = message.ends_with("?");

    if message.is_empty() {
        "Fine. Be that way!"
    } else if is_question {
        if yells {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else {
        if yells {
            "Whoa, chill out!"
        } else {
            "Whatever."
        }
    }
}
