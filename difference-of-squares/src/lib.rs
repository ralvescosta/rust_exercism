fn square_of_sum_first_scoluction(n: u32) -> u32 {
    let mut sqr_of_sum: u32 = 0;
    for number in 1..=n {
        sqr_of_sum += number;
    }
    sqr_of_sum.pow(2)
}

pub fn square_of_sum(n: u32) -> u32 {
    (1..=n).sum::<u32>().pow(2)
}

fn sum_of_squares_first_soluction(n: u32) -> u32 {
    let mut sum_of_sqr: u32 = 0;
    for number in 1..=n {
        sum_of_sqr += number.pow(2);
    }
    sum_of_sqr
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|x| x.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
