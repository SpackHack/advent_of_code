use std::fs;

fn main() {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    let input = fs::read_to_string("./inputs/day_2.txt").unwrap();

    for line in input.lines() {
        let instruction: Vec<&str> = line.split(' ').collect();

        match instruction[0] {
            "forward" => {
                pos += instruction[1].parse::<i32>().unwrap();
            }
            "down" => {
                depth += instruction[1].parse::<i32>().unwrap();
            }
            "up" => {
                depth -= instruction[1].parse::<i32>().unwrap();
            }
            _ => {
                println!("Unclear instruction");
            }
        }
    }
    println!("Position multiplying = {}", pos * depth);

    pos = 0;
    depth = 0;

    for line in input.lines() {
        let instruction: Vec<&str> = line.split(' ').collect();

        match instruction[0] {
            "forward" => {
                pos += instruction[1].parse::<i32>().unwrap();
                depth += instruction[1].parse::<i32>().unwrap() * aim;
            }
            "down" => {
                aim += instruction[1].parse::<i32>().unwrap();
            }
            "up" => {
                aim -= instruction[1].parse::<i32>().unwrap();
            }
            _ => {
                println!("Unclear instruction");
            }
        }
    }
    println!("Position with Aim multiplying = {}", pos * depth);
}
