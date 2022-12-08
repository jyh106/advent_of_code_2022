use std::{fs, collections::HashMap};

pub fn run() {
    let file_input: String = fs::read_to_string("src/inputs/day3_puzzle_input.txt").unwrap();
    let rucksacks: Vec<Vec<char>> = transform_input_to_vec(file_input);
    let shared_items: Vec<char> = get_shared_items(rucksacks);
    
    println!("priority total {:?}", get_priority_total(shared_items));
}

fn get_priority_total(shared_items: Vec<char>) -> usize {
    let mut total = 0;
    let priority_map = generate_priority_map();
    for item in shared_items {
        total = total + priority_map[&item];
    }
    return total;
}

fn generate_priority_map() -> HashMap<char, usize> {
    let mut priority_map:HashMap<char, usize> = HashMap::new();
    let alphabet:Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    for i in 0..26 {
        priority_map.insert(alphabet[i], i + 1);
        priority_map.insert(alphabet[i].to_ascii_uppercase(), 27 + i);
    }
    return priority_map;
}

fn transform_input_to_vec(input: String) ->  Vec<Vec<char>> {
    let mut rucksacks: Vec<Vec<char>> = vec![];
    for rucksack in input.lines() {
        let split_rucksack: Vec<char> = rucksack.chars().collect();
        rucksacks.push(split_rucksack);
    }
    return rucksacks;
}

fn get_shared_items(rucksacks: Vec<Vec<char>>) -> Vec<char> {
    let mut shared_items: Vec<char> = vec![];

    for rucksack in &rucksacks {
        let first_compartment = &rucksack[0..rucksack.len()/2];
        let second_compartment = &rucksack[rucksack.len()/2..rucksack.len()];
        
        for item in first_compartment {
            if second_compartment.contains(item) {
                shared_items.push(item.clone());
                break;
            }
        }
    };

    println!("shared_items length: {}, rucksacks length: {}", shared_items.len(), rucksacks.len());
    return shared_items;
}

#[cfg(test)]
mod day3_puzzle_test {
    use super::*;
    #[test]
    fn gets_shared_items_correctly() {
        let rucksacks_strings: Vec<&str> = vec!["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg", "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT"];
        let expected_shared_items: Vec<char> = vec!['p', 'L', 'P', 'v', 't', 's'];
        let mut index = 0;
        for rucksack in rucksacks_strings {
            let rucksack_char: Vec<char> = rucksack.chars().collect();
            let rucksacks = vec![rucksack_char];
            let shared_items = get_shared_items(rucksacks);
            assert_eq!(shared_items, vec![expected_shared_items[index]]);
            index = index + 1;
        }
    }

    #[test]
    fn gets_priority_value_correctly() {
        let priority_map = generate_priority_map();
        assert_eq!(priority_map[&'a'], 1);
        assert_eq!(priority_map[&'p'], 16);
        assert_eq!(priority_map[&'z'], 26);
        assert_eq!(priority_map[&'A'], 27);
        assert_eq!(priority_map[&'L'], 38);
        assert_eq!(priority_map[&'Z'], 52);
    }

    #[test]

    fn gets_priority_total_correctly() {
        let shared_items: Vec<char> = vec!['p', 'L', 'P', 'v', 't', 's'];
        assert_eq!(get_priority_total(shared_items), 157);
    }
}