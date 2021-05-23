fn sum_of_multiples_fist_implementation(limit: u32, factors: &[u32]) -> u32 {
    let mut sum_of_multiples: u32 = 0;

    for x in 0..limit {
        for factor in factors {
            if factor == &0 {
                continue;
            }
            if x % factor == 0 {
                sum_of_multiples += x;
            }
        }
    }

    sum_of_multiples
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|x| factors.iter().filter(|y| **y != 0).any(|z| x % z == 0))
        .sum()
}
