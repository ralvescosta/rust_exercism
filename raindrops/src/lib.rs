pub fn raindrops(n: u32) -> String {
    let drops = [
        (3, "Pling"),
        (5, "Plang"),
        (7, "Plong")
    ];

    let raindrop = drops.iter()
        .filter(|(factor, _)| n % factor == 0) // filter all drop if not divisible
        .map(|&(_, sound)| sound) // create a vector of sound
        .collect::<String>();

    if raindrop.is_empty() {
        return n.to_string();
    }

    return raindrop;
}
