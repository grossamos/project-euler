fn is_multiple(possible_multiple: i32, divisors: &[i32]) -> bool {
    for div in divisors {
        if possible_multiple % div == 0 {
            return true
        }
    }
    false
}

pub fn sum_of_lower_multiples(upper_bound: i32, divisors: &[i32]) -> i32 {
    let mut sum = 0;

    for num in 1..upper_bound {
        if is_multiple(num, divisors) {
            sum += num;
        }
    }

    sum
}

mod tests {
    use crate::{is_multiple, sum_of_lower_multiples};


    #[test]
    fn identifies_multiples_of_five_and_three() {
        assert!(is_multiple(15, &[5, 3]));
        assert!(is_multiple(10, &[5, 3]));
        assert!(is_multiple(9, &[5, 3]));
        assert!(is_multiple(3, &[5, 3]));
        assert!(is_multiple(5, &[5, 3]));
        assert!(!is_multiple(11, &[5, 3]));
        assert!(!is_multiple(1, &[5, 3]));
        assert!(!is_multiple(22, &[5, 3]));
    }

    #[test]
    fn identified_all_lower_multipes_of_five_and_three() {
        assert_eq!(sum_of_lower_multiples(10, &[5, 3]), 23)
    }
}
