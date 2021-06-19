pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = vec![];
    for c in string.chars() {
        match c {
            '[' => brackets.push(']'),
            '{' => brackets.push('}'),
            '(' => brackets.push(')'),
            ']' | '}' | ')' if brackets.pop() != Some(c) => return false,
            _ => continue,
        }
    }

    brackets.is_empty()
}
