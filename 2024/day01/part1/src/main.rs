use std::{env, fs::read_to_string};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = args[1].clone();

    print!("{:?}", read_distances(input_file_path));
}

fn read_distances(input_file_path: String) -> i32 {
    let mut distances_left: Vec<i32> = Vec::new();
    let mut distances_right: Vec<i32> = Vec::new();

    for line in read_to_string(input_file_path).unwrap().lines() {
        let line = line.to_string();
        let splitted_line: Vec<&str> = line.split("   ").collect();
        distances_left.push(splitted_line[0].parse().unwrap());
        distances_right.push(splitted_line[1].parse().unwrap());
    }

    assert!(distances_left.len() == distances_right.len());

    distances_left.sort();
    distances_right.sort();

    let mut total_distance = 0;

    for index in 0..distances_right.len() {
        total_distance += (distances_left[index] - distances_right[index]).abs();
    }

    total_distance
}
