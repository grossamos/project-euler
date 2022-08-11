fn check_attempt(mut path: u64) -> bool {
    path.count_ones() == 20
}

fn count_attempts() {
}
#[cfg(test)]
mod tests {
    use crate::check_attempt;


    #[test]
    fn can_check_attempt() {
        assert!(check_attempt(0b0000_0000_0000_0000_0000_1111_1111_1111_1111_1111_0000_0000_0000_0000_0000_0000));
    }
}
