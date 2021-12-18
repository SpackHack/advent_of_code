use std::{borrow::Borrow, fs};

fn main() {
    let input = fs::read_to_string("./inputs/day_3.txt").unwrap();

    let mut most_common: Vec<i32> = vec![0; 12];
    let mut gamma_rate = Vec::new();
    let mut epsilon_rate = Vec::new();

    for line in input.lines() {
        for (index, char) in line.chars().enumerate() {
            match char {
                '0' => {
                    most_common[index] -= 1;
                }
                '1' => {
                    most_common[index] += 1;
                }
                _ => {}
            }
        }
    }

    for bit in most_common.iter() {
        if bit.is_positive() {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }

    let gamma_rate_number =
        isize::from_str_radix(gamma_rate.iter().clone().collect::<String>().borrow(), 2).unwrap();
    let epsilon_rate_number =
        isize::from_str_radix(epsilon_rate.iter().clone().collect::<String>().borrow(), 2).unwrap();

    let result = gamma_rate_number * epsilon_rate_number;

    println!("Result = {}", result);

    let mut input_numbers: Vec<&str> = input.lines().into_iter().collect();

    for i in 0..12 {
        let mut zeros: Vec<&str> = Vec::new();
        let mut ones: Vec<&str> = Vec::new();

        for line in input_numbers.into_iter() {
            if line.chars().nth(i).unwrap() == '1' {
                ones.push(line.clone());
            } else {
                zeros.push(line.clone());
            }
        }
        if zeros.len() > ones.len() {
            input_numbers = zeros;
        } else {
            input_numbers = ones;
        }

        if input_numbers.len() == 1 {
            break;
        }
    }

     let oxygen_generator_rating = isize::from_str_radix(input_numbers[0], 2).unwrap();

    let mut input_numbers: Vec<&str> = input.lines().into_iter().collect();

    for i in 0..12 {
        let mut zeros: Vec<&str> = Vec::new();
        let mut ones: Vec<&str> = Vec::new();

        for line in input_numbers.into_iter() {
            if line.chars().nth(i).unwrap() == '1' {
                ones.push(line.clone());
            } else {
                zeros.push(line.clone());
            }
        }
        if zeros.len() <= ones.len() {
            input_numbers = zeros.clone();
        } else {
            input_numbers = ones.clone();
        }

        if input_numbers.len() == 1 {
            break;
        }
    }

    let co2_scrubber_rating = isize::from_str_radix(input_numbers[0], 2).unwrap();

    println!(
        "life support rating  {} * {} = {}",
        oxygen_generator_rating,
        co2_scrubber_rating,
        (oxygen_generator_rating * co2_scrubber_rating)
    )
}
