pub fn brackets_are_balanced(string: &str) -> bool {
    let mut bracket = 0;
    let mut braces = 0;
    let mut parentheses = 0;
    for c in string.chars() {
        match c {
            '[' => bracket += 1,
            ']' => bracket -= 1,
            '{' => braces += 1,
            '}' => braces -= 1,
            '(' => parentheses += 1,
            ')' => parentheses -= 1,
            _ => continue,
        }
    }

    if bracket == 0 && braces == 0 && parentheses == 0 {
        return true;
    }

    false
}
