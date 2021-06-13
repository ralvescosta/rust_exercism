pub fn is_armstrong_number(num: u32) -> bool {
    let to_string = num.to_string();
    let amount = to_string.len() as u32;
    let sum: u32 = to_string
        .chars()
        .map(|i| i.to_string().parse::<u32>().unwrap().pow(amount))
        .sum();
    return sum == num;
}
