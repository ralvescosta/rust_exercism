pub fn abbreviate(phrase: &str) -> String {
    let to_string = phrase.to_string();
    let mut normalize = String::new();
    for (i, c) in to_string.chars().into_iter().enumerate() {
        match c {
            '_' => normalize += "",
            '-' => normalize += " ",
            ',' => normalize += "",
            _ if i >= 1
                && c.is_uppercase()
                && to_string[i - 1..i].chars().next().unwrap().is_lowercase() =>
            {
                normalize += " ";
                normalize += &c.to_string();
            }
            _ => normalize += &c.to_string(),
        }
    }

    normalize
        .split(" ")
        .filter(|&s| !s.is_empty()) //remove the empty index's
        .map(|s| s.chars().next().unwrap().to_ascii_uppercase())
        .collect::<String>()
}
