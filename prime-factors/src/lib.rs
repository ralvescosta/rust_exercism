fn is_prime(n: u64) -> bool {
    let mut count = 0;
    for j in 1..=n {
        if n % j == 0 {
            count += 1;
        }
    }
    if count != 2 {
        return false;
    }
    return true;
}

fn recursive(n: u64, d: u64, v: &mut Vec<u64>) -> bool {
    if n == 1 {
        return true;
    }

    if n % d == 0 {
        v.push(d);
        return recursive(n / d, d, v);
    } else {
        return false;
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut facts = vec![];

    for i in 2.. {
        if is_prime(i) {
            let r = recursive(n, i, &mut facts);
            if r {
                break;
            }
        }
        if i == n {
            break;
        }
    }
    return facts;
}
