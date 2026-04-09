pub fn sieve_of_eratosthenes(n: u64) -> Vec<u64> {
    if n < 2 {
        return Vec::new();
    }

    let n_us = n as usize;
    let mut is_prime = vec![true; n_us + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut p: usize = 2;
    while (p * p) <= n_us {
        if is_prime[p] {
            let mut multiple = p * p;
            while multiple <= n_us {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
        p += 1;
    }

    (2..=n_us)
        .filter(|i| is_prime[*i])
        .map(|i| i as u64)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::sieve_of_eratosthenes;

    #[test]
    fn primes_up_to_30() {
        assert_eq!(
            sieve_of_eratosthenes(30),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    fn n_zero() {
        assert_eq!(sieve_of_eratosthenes(0), Vec::<u64>::new());
    }

    #[test]
    fn n_one() {
        assert_eq!(sieve_of_eratosthenes(1), Vec::<u64>::new());
    }

    #[test]
    fn n_two() {
        assert_eq!(sieve_of_eratosthenes(2), vec![2]);
    }
}
