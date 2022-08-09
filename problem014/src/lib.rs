use std::collections::HashMap;

fn generate_next_collatz_item(base: u64) -> u64 {
    if base % 2 == 0 {
        base / 2
    } else {
        3 * base + 1
    }
}

fn get_length_of_chain(mut start: u64, known_chains: &mut HashMap<u64, usize>) -> usize {
    let mut chain = vec![start];
    while start != 1 && !known_chains.contains_key(&start) {
        start = generate_next_collatz_item(start);
        chain.push(start);
    }
    let starting_point = *known_chains.get(&start).unwrap_or(&1);
    for i in 0..chain.len() {
        known_chains.insert(chain[i], chain.len() - i - 1 + starting_point);
    }
    chain.len()
}

pub fn get_largest_sequence_until_limit(limit: u64) -> u64 {
    let mut largest_seq = 0;
    let mut seq_starter = 0;
    let mut known_chains = HashMap::new();
    for i in 2..limit + 1 {
        let len = get_length_of_chain(i, &mut known_chains);
        if largest_seq < len {
            largest_seq = len;
            seq_starter = i;
        }
    }
    seq_starter
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{generate_next_collatz_item, get_length_of_chain, get_largest_sequence_until_limit};

    #[test]
    fn can_generate_next_collatz_item() {
        assert_eq!(generate_next_collatz_item(13), 40);
        assert_eq!(generate_next_collatz_item(40), 20);
        assert_eq!(generate_next_collatz_item(20), 10);
        assert_eq!(generate_next_collatz_item(10), 5);
        assert_eq!(generate_next_collatz_item(5), 16);
        assert_eq!(generate_next_collatz_item(16), 8);
        assert_eq!(generate_next_collatz_item(8), 4);
        assert_eq!(generate_next_collatz_item(2), 1);
    }

    #[test]
    fn can_get_length_of_chain() {
        let mut known_chains = HashMap::new();
        let len = get_length_of_chain(13, &mut known_chains);
        assert_eq!(len, 10 as usize);
        assert_eq!(known_chains[&13], 10);
        assert_eq!(known_chains[&40], 9);
        assert_eq!(known_chains[&16], 5);
        assert_eq!(known_chains[&1], 1);

        let mut known_chains = HashMap::new();
        let len = get_length_of_chain(3, &mut known_chains);
        assert_eq!(len, 8);

        let mut known_chains = HashMap::new();
        let len = get_length_of_chain(2, &mut known_chains);
        assert_eq!(len, 2);
    }

    #[test]
    fn can_get_longest_chain() {
        assert_eq!(get_largest_sequence_until_limit(5), 3);
        assert_eq!(get_largest_sequence_until_limit(3), 3);
    }
}
