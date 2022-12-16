mod day_one_puzzle;
mod day_two_puzzle;
mod day_three_puzzle;
mod day_four_puzzle;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    let puzzle_commands = [day_one_puzzle::run, day_two_puzzle::run, day_three_puzzle::run, day_four_puzzle::run];

    let day = args.next().unwrap().parse::<usize>().unwrap();

    puzzle_commands[day - 1]();
}
