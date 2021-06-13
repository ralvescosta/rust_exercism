fn recursive(n: u64, d: u64, v: &mut Vec<u64>) -> bool {
    if n == 1 {
        return true;
    }

    if n % d == 0 {
        v.push(d);
        recursive(n / d, d, v);
    } else {
        return false;
    }

    return true;
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut facts = vec![];

    for i in 2.. {
        if recursive(n, i, &mut facts) {
            break;
        }
        if i == n {
            break;
        }
    }
    return facts;
}
