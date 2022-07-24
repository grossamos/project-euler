use float_eq::float_eq;

pub fn find_smallest_multiple(divison_limit: u32, second_divisor: u32) -> u32 {
    let base = get_base(divison_limit);
    let mut smallest_multiple = base;

    while !test_divisiblity(smallest_multiple, second_divisor, divison_limit) {
        smallest_multiple += base;
    }
    smallest_multiple
}

fn get_base(limit: u32) -> u32 {
    let mut base = 2;
    let mut prime_generator = Primes::new();

    while prime_generator.get_recent_prime() < limit {
        base *= prime_generator.get_recent_prime();
        prime_generator.next();
    }
    base
}

fn test_divisiblity(base: u32, constant_divisor: u32, limit: u32) -> bool {
    for divisor in 2..limit + 1 {
        if !float_eq!((base as f64 / divisor as f64) % constant_divisor as f64, 0.0, abs <= 1.0) {
            return false;
        }
    }
    true
}

struct Primes {
    primes: Vec<u32>,
    counter: u32,
}

impl Primes {
    fn new() -> Primes {
        Primes { primes: vec![2], counter: 2 }
    }

    fn get_recent_prime(&self) -> u32 {
        self.primes[self.primes.len() - 1]
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.counter += 1;
            if !self.primes.iter().any(|x| self.counter % x == 0) {
                self.primes.push(self.counter);
                return Some(self.counter);
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::{Primes, get_base, test_divisiblity, find_smallest_multiple};

    // even 
    // factors: 2, 2, 3, 5, 7 ... (primes)
    #[test]
    fn gets_basic_increments() {
        assert_eq!(get_base(10), 420);
        assert_eq!(get_base(20), 19399380);
    }

    #[test]
    fn generates_primes() {
        let prime_generator = Primes::new();
        assert_eq!(prime_generator.take(5).collect::<Vec<u32>>(), vec![3, 5, 7, 11, 13]);
    }

    #[test]
    fn does_divisibility_test() {
        assert!(test_divisiblity(2520, 1, 10));
        assert!(!test_divisiblity(8888, 1, 10));
    }

    #[test]
    fn finds_smallest_multiple() {
        assert_eq!(find_smallest_multiple(10, 1), 2520);
    }
}
