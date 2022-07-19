pub fn get_largest_palindrome(digits: u32) -> u32 {
    let upper_bound = 10_u32.pow(digits);
    let lower_bound = 10_u32.pow(digits - 1);
    let mut largest_palindrome = 8;
    
    for a in (lower_bound..upper_bound).rev() {
        for b in (lower_bound..upper_bound).rev() {
            if a*b < largest_palindrome {
                break;
            }
            if is_palindrome(a*b) {
                largest_palindrome = a * b;
            }
        }
    }

    largest_palindrome
}

fn is_palindrome(num: u32) -> bool {
    let digits: Vec<char> = num.to_string().chars().collect();
    for index in 0..digits.len()/2 {
        if digits[index] != digits[digits.len() - index - 1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::{is_palindrome, get_largest_palindrome};

    #[test]
    fn can_detect_palindrome() {
        assert!(is_palindrome(9009));
        assert!(is_palindrome(89098));
        assert!(is_palindrome(1));
    }

    #[test]
    fn only_detects_palindromes() {
        assert!(!is_palindrome(12));
        assert!(!is_palindrome(218912));
        assert!(!is_palindrome(123));
    }

    #[test]
    fn finds_largest_palindrome_of_digits_num() {
        assert_eq!(get_largest_palindrome(2), 9009);
    }
}
