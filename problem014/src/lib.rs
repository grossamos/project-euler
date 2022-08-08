use std::collections::HashMap;

fn generate_next_collatz_item(base: u32) -> u32 {
    if base % 2 == 0 {
        base / 2
    } else {
        3 * base + 1
    }
}

fn get_length_of_chain(mut start: u32, known_chains: &mut HashMap<u32, usize>) -> usize {
    let mut chain = vec![start];
    while start != 1 || known_chains.contains_key(&start) {
        start = generate_next_collatz_item(start);
        chain.push(start);
    }
    let starting_point = *known_chains.get(&start).unwrap_or(&1);
    for i in 0..chain.len() {
        known_chains.insert(chain[i], i + starting_point);
    }
    println!("{:?}", chain);
    chain.len()
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{generate_next_collatz_item, get_length_of_chain};

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
        assert_eq!(len, 10);
        assert_eq!(known_chains[&13], 10);
        assert_eq!(known_chains[&40], 9);
        assert_eq!(known_chains[&16], 5);
        assert_eq!(known_chains[&1], 1);
    }
}
