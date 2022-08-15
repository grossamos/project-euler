fn check_attempt(path: u64, grid_size: u32) -> bool {
    if path % 10000000000 == 0 {
        println!("{}, {}", path, u64::MAX);
    }
    path.count_ones() == grid_size
}

pub fn count_attempts(grid_size: u32) -> usize {
    //let mut count = 0;
    //for i in (u64::MIN + 2_u64.pow(grid_size + 1)..u64::MAX - 2_u64.pow(grid_size - 1)).step_by(2_usize.pow(64 - grid_size * 2)) {
        //if check_attempt(i, grid_size) {
            //count += 1;
        //}
    //}
    (u64::MIN + 2_u64.pow(grid_size + 1)..u64::MAX - 2_u64.pow(grid_size - 1)).step_by(2_usize.pow(64 - grid_size * 2)).filter(|x| check_attempt(*x, grid_size)).count()

    //count
}

#[cfg(test)]
mod tests {
    use crate::{check_attempt, count_attempts};


    #[test]
    fn can_check_attempt() {
        assert!(check_attempt(0b0000_0000_0000_0000_0000_1111_1111_1111_1111_1111_0000_0000_0000_0000_0000_0000, 20));
    }

    #[test]
    fn can_count_attempts() {
        assert_eq!(count_attempts(2), 6);
    }
}
