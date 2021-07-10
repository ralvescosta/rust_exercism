use std::u64;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = (2..upper_bound).map(|p| p).collect();
    let mut index: u64 = 2;
    loop {
        if let Some(v) = mark(&mut primes, index) {
            return v;
        }
        index += 1;
    }
}

fn mark(prime: &mut Vec<u64>, index: u64) -> Option<Vec<u64>> {
    let mut amount = 0;
    for p in prime.into_iter() {
        if *p == index {
            continue;
        }

        if *p % index == 0 {
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
