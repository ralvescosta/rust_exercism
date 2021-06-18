pub fn reply(message: &str) -> &str {
    match message.trim() {
        msg if msg.to_uppercase() == msg
            && msg.ends_with("?")
            && msg.matches(char::is_alphabetic).count() > 0 =>
        {
            "Calm down, I know what I'm doing!"
        }
        msg if msg.ends_with("?") => "Sure.",
        msg if msg.is_empty() => "Fine. Be that way!",
        msg if msg.to_uppercase() == msg && msg.matches(char::is_alphabetic).count() > 0 => {
            "Whoa, chill out!"
        }
        _ => "Whatever.",
    }
}
