use std::fs;

fn main() {
    let numbers: Vec<u32> = fs::read_to_string("./inputs/day_1.txt").unwrap().lines().map(|line| line.parse::<u32>().unwrap()).collect();

    let mut inc = -1;
    let mut inc_sum = -1;

    let mut previous_number = 0;
    let mut previous_number_sum = 0;

    for number in numbers.iter() {
        if previous_number < *number {
            inc += 1;
        }
        previous_number = *number;
    }

    let mut sum: u32;
    for index in 0..(numbers.len() - 2) {

        sum = numbers[index..(index + 3)].iter().sum();

        if previous_number_sum < sum {
            inc_sum += 1;
        }
        previous_number_sum = sum;
    }

    println!("Number Increased {} times", inc);
    println!("Sum Number Increased {} times", inc_sum);
}
