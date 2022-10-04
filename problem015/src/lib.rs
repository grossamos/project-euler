use std::collections::HashMap;

struct DynamicCache {
    known_waypoints: HashMap<(u64, u64), u64>
}

impl DynamicCache {
    fn calc_waypoint_sum(&mut self, x: u64, y: u64) -> u64 {
        match self.known_waypoints.get(&(x, y)) {
            Some(waypoint_sum) => {
                *waypoint_sum
            },
            None => {
                let waypoint_sum = if x == 0 && y == 0 {
                    0
                } else if x == 0 || y == 0 {
                    1
                } else {
                    println!("{}, {}", x, y);
                    self.calc_waypoint_sum(x - 1, y) + self.calc_waypoint_sum(x, y - 1)
                };
                self.known_waypoints.insert((x, y), waypoint_sum);
                waypoint_sum
            }
        }
    }
}

pub fn calc_waypoint_sum(x: u64, y: u64) -> u64 {
    let mut cache = DynamicCache {known_waypoints: HashMap::new()};
    cache.calc_waypoint_sum(x, y)
}

#[cfg(test)]
mod tests {
    use crate::calc_waypoint_sum;

    #[test]
    fn can_calculate_waypoint_sum_base_cases() {
        assert_eq!(0, calc_waypoint_sum(0, 0));
        assert_eq!(1, calc_waypoint_sum(1, 0));
        assert_eq!(1, calc_waypoint_sum(0, 1));
        assert_eq!(1, calc_waypoint_sum(0, 10));
    }

    #[test]
    fn can_calculate_waypoint_sum() {
        assert_eq!(2, calc_waypoint_sum(1, 1));
        assert_eq!(3, calc_waypoint_sum(1, 2));
        assert_eq!(6, calc_waypoint_sum(2, 2));
        assert_eq!(20, calc_waypoint_sum(3, 3));
    }
}
