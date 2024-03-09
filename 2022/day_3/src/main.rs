use utils::read_input_file;
use std::collections::HashMap;

fn main() {
    let lines = read_input_file("input.txt").unwrap();

    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}

fn convert_ascii_to_int(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        'A'..='Z' => c as i32 - 'A' as i32 + 27,
        _ => 0,
    }
}

fn part1(lines: Vec<String>) -> i32 {
    let mut ascii_values: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let mut ascii_line: Vec<i32> = Vec::new();
        for c in line.chars() {
            ascii_line.push(convert_ascii_to_int(c)); 
        }
        ascii_values.push(ascii_line);
    }

    let mut sum = 0;
    
    for i in 0..ascii_values.len() {
        let mid = ascii_values[i].len() / 2;
        let mut left = HashMap::new();
    
        for j in 0..mid {
            left.insert(ascii_values[i][j], false);
        }

        let mut duplicate_score = 0;

        for j in mid..ascii_values[i].len() {
            if left.contains_key(&ascii_values[i][j]) && !left[&ascii_values[i][j]] {
                duplicate_score += &ascii_values[i][j];
                *left.get_mut(&ascii_values[i][j]).unwrap() = true;
            }
        }

        sum += duplicate_score;
    }

    sum
}

fn part2(lines: Vec<String>) -> i32 {
    let mut ascii_values: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let mut ascii_line: Vec<i32> = Vec::new();
        for c in line.chars() {
            ascii_line.push(convert_ascii_to_int(c)); 
        }
        ascii_values.push(ascii_line);
    }

    let mut sum = 0;

    for i in (0..=ascii_values.len() - 2).step_by(3) {
        let mut group = HashMap::new(); 

         for j in 0..3 {
            let mut seen_in_group = HashMap::new();

            for k in 0..ascii_values[i + j].len() {
                if !seen_in_group.contains_key(&ascii_values[i + j][k]) {
                    *group.entry(ascii_values[i + j][k]).or_insert(0) += 1;
                    seen_in_group.insert(ascii_values[i + j][k], true);
                }
            }
        }

        for (key, value) in group.iter() {
            if *value == 3 {
                sum += key;
            }
        }
    }

    sum
}
