pub fn get_largest_prime_divisor(base: u64) -> Option<u64> {
    let mut base = base;

    let factors = PrimeFactor::new();
    for factor in factors {
        while base % factor == 0 {
            base /= factor;
        }
        if base == 1 {
            return Some(factor);
        }
    }
    None
}

pub struct PrimeFactor {
    factors: Vec<u64>,
    counter: u64,
}

impl PrimeFactor {
    pub fn new() -> PrimeFactor {
        PrimeFactor{factors: vec![2], counter: 2}
    }
}

impl Iterator for PrimeFactor {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.counter += 1;
            if !self.factors.iter().any(|x| self.counter % x == 0) {
                self.factors.push(self.counter);
                return Some(self.counter);
            }
        }

    }
}


#[cfg(test)]
mod test {
    use crate::get_largest_prime_divisor;


    #[test]
    fn finds_largest_prime_divisor() {
        assert_eq!(get_largest_prime_divisor(13195).unwrap(), 29);
    }
}
