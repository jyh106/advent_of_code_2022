use std::fs;

pub fn run() {
    let file_input: String = fs::read_to_string("src/inputs/puzzle1_input.txt").unwrap();
    let elves_calories = file_input.split("\n\n").collect();
    let totals: Vec<i32> = get_totals(elves_calories);
    let largest = find_largest(&totals);
    let second_largest = find_second_largest(&totals, &largest);
    let third_largest = find_second_largest(&totals, &second_largest);
    let total = &largest + &second_largest + &third_largest;
    println!("total calories of top 3 elves is {}", total);
}

fn find_second_largest(nums: &Vec<i32>, largest: &i32) -> i32 {
    let mut second_largest: i32 = 0;
    for num in nums {
        if (num < largest) && (num > &second_largest) {
            second_largest = *num;
        }
    }
    return second_largest;
}

fn find_largest(nums: &Vec<i32>) -> i32 {
    let mut largest = 0;
    for num in nums {
        if num > &largest {
            largest = *num;
        }
    }
    return largest;
}

fn get_totals(nums: Vec<&str>) -> Vec<i32> {
    let mut totals: Vec<i32> = vec![];
    for nums_string in nums {
        let mut total = 0;
       for num in nums_string.lines() {
            total = total + num.parse::<i32>().unwrap();
       }
       totals.push(total);
    }
    return totals;
}