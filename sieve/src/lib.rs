use std::{borrow::BorrowMut, u64};

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = (2..upper_bound).map(|p| p).collect();
    let value: &mut u64 = &mut 2;
    loop {
        if let Some(v) = mark(&mut primes, value) {
            return v;
        }
    }
}

fn mark<'a>(prime: &'a mut Vec<u64>, value: &'a mut u64) -> Option<Vec<u64>> {
    let mut amount = 0;

    for (i, p) in prime.into_iter().enumerate() {
        if *p == *value {
            continue;
        }

        if *p % *value == 0 {
            amount += *p;
            *p = 0;
        }
    }

    if amount == 0 {
        return Some(
            prime
                .iter()
                .filter(|&&p| p != 0)
                .map(|&p| p)
                .collect::<Vec<u64>>(),
        );
    }
    return None;
}
