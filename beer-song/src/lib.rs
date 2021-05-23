pub fn verse(n: u32) -> String {
    let sub = n.wrapping_sub(1);
    match n {
        0 => {
            String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
        },
        1 => {
            String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n")
        },
        _ => {
            let mut amount = String::new();
            if sub > 1 {
                amount.push_str("bottles");
            }
            else {
                amount.push_str("bottle");
            }
            let formatted = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} {} of beer on the wall.\n", n, n, sub, amount);
            String::from(formatted.as_str())
        }
    } 
}

pub fn sing(start: u32, end: u32) -> String {
    let mut sign = String::new();
    for i in (end..=start).rev() {
      sign.push_str(verse(i).as_str());
    }
    sign
}
