pub fn nth(n: u32) -> u32 {
    let n = n + 1;
    let limit = 1_000_000;
    let mut sieve = vec![true; limit];
    let mut count = 0;

    sieve[0] = false;
    sieve[1] = false;

    for prime in 2..limit {
        if !sieve[prime] {
            continue;
        }
        count += 1;
        if count == n {
            return prime as u32;
        }

        for multiple in ((prime * prime)..limit).step_by(prime) {
            sieve[multiple] = false;
        }
    }

    panic!("impossible prime number for this algorithm");
}
