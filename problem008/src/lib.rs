fn get_individual_nums<'a>(num_str: &'a str, length: usize) -> Vec<&'a str> {
    let mut individual_nums = vec![];
    for index in 0..(num_str.len() - length + 1) {
        individual_nums.push(&num_str[index..index+length])
    }

    individual_nums
}

fn product_of_digits(number: &str) -> u64 {
    let mut product = 1;
    for digit in number.chars() {
        product *= digit.to_digit(10).unwrap() as u64
    }
    product
}

pub fn get_greatest_sum_num(number: &str, length: usize) -> u64 {
    let mut largest_num = 0;
    for sub_str in get_individual_nums(number, length) {
        let prod = product_of_digits(sub_str);
        if prod > largest_num {
            largest_num = prod;
        }
    }
    largest_num
}

#[cfg(test)]
mod test {
    use crate::{get_individual_nums, product_of_digits, get_greatest_sum_num};

    // 
    #[test]
    fn generates_stream_of_uints() {
        assert_eq!(get_individual_nums("123456789", 5), vec!["12345", "23456", "34567", "45678", "56789"]);
        assert_eq!(get_individual_nums("12345678", 6), vec!["123456", "234567", "345678"]);
    }

    #[test]
    fn generates_product_of_digits() {
        assert_eq!(product_of_digits("9989"), 9 * 9 * 8 * 9);
        assert_eq!(product_of_digits("5372"), 5 * 3 * 7 * 2);
    }

    #[test]
    fn finds_greatest_num_in_str() {
        assert_eq!(get_greatest_sum_num("123499891234", 4), 9 * 9 * 8 * 9);
    }
}
