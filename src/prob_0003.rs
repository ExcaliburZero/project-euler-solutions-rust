pub fn solve() -> String {
    let mut remaining: i64 = 600851475143;

    let mut known_primes = vec![2];

    let mut n = 2;
    let mut largest_prime_factor = 0;
    while remaining >= n {
        n += 1;

        if known_primes.iter().any(|p| n % p == 0) {
            continue;
        }
        known_primes.push(n);

        if remaining % n != 0 {
            continue;
        }

        remaining /= n;
        largest_prime_factor = n;
    }

    largest_prime_factor.to_string()
}
