use std::fs;

pub fn run() {
    let file_input: String = fs::read_to_string("src/inputs/day2_puzzle_input.txt").unwrap();
    let mut total_score = 0;
    for line in file_input.lines() {
        let play_round = line.split(" ").collect::<Vec<&str>>();
        total_score = total_score + my_score(play_round[0], play_round[1]);
    }
    println!("My total expected score is {}", total_score);
}

fn decrpyt(action: &str) -> String {
    if action == "X" || action == "A" {
        return String::from("Rock");
    }
    if action == "Z" || action == "C" {
        return String::from("Scissors");
    }
    if action == "Y" || action == "B" {
        return String::from("Paper");
    } else {
        return String::from("unknown");
    }
}

fn encrypt_my_action(action: &str) -> String {
    if action == "Scissors" {
        return "Z".to_string();
    } else if action == "Paper" {
        return "Y".to_string();
    } else if action == "Rock" {
        return "X".to_string();
    } else {
        return "unknown".to_string();
    }
}

fn get_my_action(my_goal: &str, opponent_action: &str) -> String {
    let decrypt_oppoinent_action = decrpyt(opponent_action);
    if my_goal == "X" { // loose 
        if decrypt_oppoinent_action == "Rock" {
            return "Scissors".to_string();
        } else if decrypt_oppoinent_action == "Scissors" {
            return "Paper".to_string();
        } else if decrypt_oppoinent_action == "Paper" {
            return "Rock".to_string();
        } else {
            return "unknown".to_string();
        }
    } else if my_goal == "Y" { // draw
        return decrypt_oppoinent_action.clone();
    } else if my_goal == "Z" { // win
        if decrypt_oppoinent_action == "Rock" {
            return "Paper".to_string();
        } else if decrypt_oppoinent_action == "Scissors" {
            return "Rock".to_string();
        } else if decrypt_oppoinent_action == "Paper" {
            return "Scissors".to_string();
        } else {
            return "unknown".to_string();
        }
    } else {
        return "unknown".to_string();
    }
}

fn my_score(opponent: &str, mine: &str) -> i32 {
    let mut score = 0;
    let my_action = get_my_action(mine, opponent);
    println!("{}", &my_action);
    let opponent_action = decrpyt(&opponent);
    if my_action == "Rock" { 
        score = score + 1;
        if opponent_action == "Scissors" { 
            score = score + 6;
        }
    } else if my_action == "Paper" {
        score = score + 2;
        if opponent_action == "Rock" {
            score = score + 6;
        }
    } else if my_action == "Scissors" {
        score = score + 3;
        if opponent_action == "Paper" {
            score = score + 6;
        }
    } 
    if opponent_action == my_action {
        score = score + 3;
    }
    return score;
}