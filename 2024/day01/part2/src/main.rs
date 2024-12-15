use std::{collections::HashMap, env, fs::read_to_string};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let similarity_score = calculate_similarity_score(file_path.to_string());
    println!("{}", similarity_score);
}

fn calculate_similarity_score(file_path: String) -> i32 {
    let mut location_map = HashMap::new();
    let mut similarity_score = 0;
    let mut distances_left = Vec::new();

    println!("{}", file_path);

    for line in read_to_string(file_path).unwrap().lines() {
        let line = line.to_string();
        let splitted_line: Vec<&str> = line.split("   ").collect();
        let left_location: i32 = splitted_line[0].parse().unwrap();
        let right_location: i32 = splitted_line[1].parse().unwrap();
        distances_left.push(left_location);
        location_map
            .entry(right_location)
            .and_modify(|freq| *freq += 1)
            .or_insert(1);
    }

    for &distance in &distances_left {
        match location_map.get(&distance) {
            Some(freq) => similarity_score += distance * *freq,
            _ => (),
        }
    }

    similarity_score
}
