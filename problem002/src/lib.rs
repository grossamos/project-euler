const GOLDEN_RATIO: f64 = 1.618034;

fn get_fib_at_index(index: i32) -> i32 {
    ((GOLDEN_RATIO.powf(index.into()) - (1.0 - GOLDEN_RATIO).powf(index.into()))/5.0_f64.sqrt()) as i32
}

pub fn sum_of_evens(limit: i32) -> i32 {
    let mut sum = 0;
    let mut index = 3;

    loop {
        let fib_val = get_fib_at_index(index);
        if fib_val > limit {
            return sum;
        }
        sum += fib_val;
        index += 3;
    }
}

#[cfg(test)]
mod test {
    use crate::{get_fib_at_index, sum_of_evens};

    #[test]
    fn correctly_calculates_fib_sequence() {
        assert_eq!(get_fib_at_index(1), 1);
        assert_eq!(get_fib_at_index(2), 1);
        assert_eq!(get_fib_at_index(3), 2);
        assert_eq!(get_fib_at_index(4), 3);
        assert_eq!(get_fib_at_index(5), 5);
        assert_eq!(get_fib_at_index(6), 8);
        assert_eq!(get_fib_at_index(7), 13);
    }

    #[test]
    fn correctly_gets_even_sum() {
        assert_eq!(sum_of_evens(12), 10);
        assert_eq!(sum_of_evens(14), 10);
    }
}
