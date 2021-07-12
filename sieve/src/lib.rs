pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = (2..=upper_bound).map(|p| p).collect();
    return mark(&mut primes, 2);
}

fn mark<'a>(prime: &'a mut Vec<u64>, value: u64) -> Vec<u64> {
    let mut amount = 0;
    let mut get_next_value = false;
    let mut next_value = 0;

    for p in prime.into_iter() {
        if *p == value {
            get_next_value = true;
            continue;
        }

        if get_next_value && *p > value {
            next_value = *p;
            get_next_value = false;
        }

        if *p % value == 0 {
            amount += *p;
            *p = 0;
        }
    }

    if amount > 0 {
        return mark(prime, next_value);
    }

    return prime
        .iter()
        .filter(|&&p| p != 0)
        .map(|&p| p)
        .collect::<Vec<u64>>();
}
