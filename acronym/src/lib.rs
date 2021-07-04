pub fn abbreviate(phrase: &str) -> String {
    phrase
        .replace("-", " ")
        .replace("_", "")
        .split(" ")
        .filter(|&s| !s.is_empty())
        .map(|s| s.chars().next().unwrap().to_ascii_uppercase())
        .collect::<String>()
}
