use std::fs;

pub fn run() {
    let file_input: String = fs::read_to_string("src/inputs/day4_puzzle_input.txt").unwrap();
    let sections = transform_input(file_input);
    let contained_count = count_contained(&sections);
    let overlapped_count = count_overlapped(&sections);
    println!("puzzle part 1 - contained count {}", contained_count);
    println!("puzzle part 2 - overlapped count {}", overlapped_count);
}

fn transform_input(input: String) -> Vec<(Vec<i32>, Vec<i32>)>{
    let mut sections: Vec<(Vec<i32>, Vec<i32>)> = vec![];
    for sections_string in input.lines() {
        let sections_string: Vec<&str> = sections_string.split(",").collect();
        let first_section_string: Vec<&str> = sections_string[0].split("-").collect();
        let second_section_string: Vec<&str> = sections_string[1].split("-").collect();
        let first_section_vec: Vec<i32> = vec![first_section_string[0].parse::<i32>().unwrap(), first_section_string[1].parse::<i32>().unwrap()];
        let second_section_vec: Vec<i32> = vec![second_section_string[0].parse::<i32>().unwrap(), second_section_string[1].parse::<i32>().unwrap()];
        sections.push((first_section_vec, second_section_vec));
    }
    return sections;
}

fn count_contained(sections: &Vec<(Vec<i32>, Vec<i32>)>) -> i32 {
    let mut contained_count = 0;
    for section in sections {
        if is_contained(section) {
            contained_count = contained_count + 1;
        }
    }
    return contained_count;
}

fn count_overlapped(sections: &Vec<(Vec<i32>, Vec<i32>)>) -> i32 {
    let mut overlapped_count = 0;
    for section in sections {
        if is_overlapped(section) {
            overlapped_count = overlapped_count + 1;
        }
    }
    return overlapped_count;
}

fn is_contained(sections: &(Vec<i32>, Vec<i32>)) -> bool {
    let (first_section, second_section) = sections; 
    if first_section[0] <= second_section[0] && first_section[1] >= second_section[1] {
        return true;
    }
    if second_section[0] <= first_section[0] && second_section[1] >= first_section[1] {
        return true;
    }
    return false;
}

fn is_overlapped(sections: &(Vec<i32>, Vec<i32>)) -> bool {
    let (first_section, second_section) = sections;

    if &first_section[0] >= &second_section[0] && &first_section[0] <= &second_section[1] {
        return true;
    }
    if &first_section[1] >= &second_section[0] && &first_section[1] <= &second_section[1] {
        return true;
    }
    if &second_section[0] >= &first_section[0] && &second_section[0] <= &first_section[1] {
        return true;
    }
    if &second_section[1] >= &first_section[0] && &second_section[1] <= &first_section[1] {
        return true
    }
    return false;
}

#[cfg(test)]
mod day_4_puzzle_test {
    use super::*;
    #[test]
    fn evaluates_is_contained_correctly() {
        let tuple = (vec![6,6], vec![4,6]);
        assert_eq!(is_contained(&tuple), true);
        let tuple = (vec![2,4], vec![6,8]);
        assert_eq!(is_contained(&tuple), false);
    }

    #[test]
    fn gets_contained_section_count_correctly() {
        let sections = vec![(vec![2,3], vec![4,5]), (vec![5,7], vec![7,9]), (vec![2,8], vec![3,7]), (vec![6,6], vec![4,6]), (vec![2,6], vec![4,8])];
        assert_eq!(count_contained(&sections), 2);
    }

    #[test]
    fn evaluates_is_overlapped_correctly() {
        assert_eq!(is_overlapped(&(vec![1,2], vec![3,5])), false);
        assert_eq!(is_overlapped(&(vec![5,7], vec![7,9])), true);
    }

    #[test]
    fn gets_overlapped_count_correctly() {
        let sections = vec![(vec![2,4], vec![6,8]),(vec![2,3], vec![4,5]), (vec![5,7], vec![7,9]), (vec![2,8], vec![3,7]), (vec![6,6], vec![4,6]), (vec![2,6], vec![4,8])];
        assert_eq!(count_overlapped(&sections), 4);
    }
}