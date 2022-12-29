use std::{fs, vec};

pub fn run() {
    let stack_1 = vec!['P'.to_string(), 'D'.to_string(), 'Q'.to_string(), 'R'.to_string(), 'V'.to_string(), 'B'.to_string(), 'H'.to_string(), 'F'.to_string()];
    let mut stack_2 = vec!['V'.to_string(), 'W'.to_string(), 'Q'.to_string(), 'Z'.to_string(), 'D'.to_string(), 'L'.to_string()];
    let mut stack_3 = vec!['C'.to_string(), 'P'.to_string(), 'R'.to_string(), 'G'.to_string(), 'Q'.to_string(), 'Z'.to_string(), 'L'.to_string(), 'H'.to_string()];
    let mut stack_4 = vec!['B'.to_string(), 'V'.to_string(), 'J'.to_string(), 'F'.to_string(), 'H'.to_string(), 'D'.to_string(), 'R'.to_string()];
    let mut stack_5 = vec!['C'.to_string(), 'L'.to_string(), 'W'.to_string(), 'Z'.to_string()];
    let mut stack_6 = vec!['M'.to_string(), 'V'.to_string(), 'G'.to_string(), 'T'.to_string(), 'N'.to_string(), 'P'.to_string(), 'R'.to_string(), 'J'.to_string()];
    let mut stack_7 = vec!['S'.to_string(), 'B'.to_string(), 'M'.to_string(), 'V'.to_string(), 'L'.to_string(), 'R'.to_string(), 'J'.to_string()];
    let mut stack_8 = vec!['J'.to_string(), 'P'.to_string(), 'D'.to_string()];
    let mut stack_9 = vec!['V'.to_string(), 'W'.to_string(), 'N'.to_string(), 'C'.to_string(), 'D'.to_string()];
    
    let file_input: String = fs::read_to_string("src/inputs/day5_puzzle_input.txt").unwrap();
    let instructions = transform_instructions(file_input);
    let mut stacks = vec![stack_1, stack_2, stack_3, stack_4, stack_5, stack_6, stack_7, stack_8, stack_9];

    // part I
    // move_stacks_one_at_a_time(&instructions, &mut stacks);
    // let top_of_stacks = get_top_of_stacks(&stacks);
    // println!("top items in stacks: {:?}", top_of_stacks);

    // part II 
    move_stacks_at_once(&instructions, &mut stacks);
    let top_of_stacks = get_top_of_stacks(&stacks);
    println!("top items in stacks: {:?}", top_of_stacks);
}

fn move_stacks_at_once(instructions: &Vec<(usize, usize, usize)>, stacks: &mut Vec<Vec<String>>) {
    for instruction in instructions {
        let (number_of_crates_to_move, from_stack, to_stack) = instruction;
       
        let sliced_moving_stack = Vec::from_iter(stacks[from_stack - 1][0..*number_of_crates_to_move].iter().cloned());

        for i in (0..*number_of_crates_to_move).rev() {
            let from = &stacks[from_stack - 1];
            stacks[from_stack - 1].remove(0);
            stacks[to_stack - 1].insert(0, sliced_moving_stack[i].clone());
        }
    }
}

fn get_top_of_stacks(stacks: &Vec<Vec<String>>) -> Vec<String> {
    let mut top_of_stacks:Vec<String> = vec![];

    for stack in 0..stacks.len() {
        top_of_stacks.push(stacks[stack][0].clone());
    }

    return top_of_stacks;
}

fn move_stacks_one_at_a_time(instructions: &Vec<(usize, usize, usize)>, stacks: &mut Vec<Vec<String>>) {
    for instruction in instructions {
        let (number_of_crates_to_move, from_stack, to_stack) = instruction;
        for _i in 0..*number_of_crates_to_move {
            let from = &stacks[from_stack - 1];
            let letter:String = from[0].clone();
            stacks[from_stack - 1].remove(0);
            stacks[to_stack -1].insert(0, letter);
        }

    }
}

fn transform_instructions(file_input: String) -> Vec<(usize, usize, usize)> {
    let mut transformed_instructions: Vec<(usize, usize, usize)> = vec![];
    for instruction in file_input.lines() {
        let parsed_instruction: Vec<&str> = instruction.split(" ").collect();
        let transformed_instruction = (parsed_instruction[1].parse::<usize>().unwrap(), parsed_instruction[3].parse::<usize>().unwrap(), parsed_instruction[5].parse::<usize>().unwrap());
        transformed_instructions.push(transformed_instruction);
    }
    return transformed_instructions;
}

#[cfg(test)]
mod day_5_puzzle_test {
    use super::*;
    #[test]
    fn move_stacks_one_at_a_time_correctly() {
        let instructions = vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)];
        let mut stacks = vec![vec![String::from("N"), String::from("Z")], vec![String::from("D"), String::from("C"), String::from("M")], vec![String::from("P")]];
        let expected_stacks = vec![vec![String::from("C")], vec![String::from("M")], vec![String::from("Z"), String::from("N"), String::from("D"), String::from("P")]];
        move_stacks_one_at_a_time(&instructions, &mut stacks);

        assert_eq!(stacks, expected_stacks);
    }

    fn collect_top_crates_correctly() {
        let instructions = vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)];
        let mut stacks = vec![vec![String::from("N"), String::from("Z")], vec![String::from("D"), String::from("C"), String::from("M")], vec![String::from("P")]];
        let expected_top_stacks = vec!["C".to_string(), "M".to_string(), "Z".to_string()];
        move_stacks_one_at_a_time(&instructions, &mut stacks);

        assert_eq!(get_top_of_stacks(&stacks), expected_top_stacks);
    }

    fn move_stacks_at_once_correctly() {
        let instructions = vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)];
        let mut stacks = vec![vec![String::from("N"), String::from("Z")], vec![String::from("D"), String::from("C"), String::from("M")], vec![String::from("P")]];
        let expected_stacks = vec![vec![String::from("M")], vec![String::from("C")], vec![String::from("D"), String::from("N"), String::from("Z"), String::from("P")]];
        move_stacks_at_once(&instructions, &mut stacks);

        assert_eq!(stacks, expected_stacks);
    }
}