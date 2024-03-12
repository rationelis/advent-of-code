use utils::read_input_file;
use std::collections::VecDeque;

fn main() {
    let lines = read_input_file("input.txt").unwrap();

    println!("Part 1: {:?}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}

fn get_crates(configuration: &[String]) -> Vec<VecDeque<i32>> {
    let stack_numbers: Vec<char> = configuration.iter().flat_map(|s| s.chars()).filter(|c| c.is_digit(10)).collect();
    
    let mut crates: Vec<VecDeque<i32>> = Vec::new();

    for _ in 0..stack_numbers.len() {
        let stack = VecDeque::new();
        crates.push(stack);
    }

    for line in configuration {
        let mut chars = line.chars().collect::<Vec<char>>(); 
        
        let mut whitespace_at = 3;
        for _ in 0..crates.len() - 1 {
            chars.remove(whitespace_at);
            whitespace_at += 3;
        }

        for i in (0..chars.len()).step_by(3) {
            let index = i / 3;
            if chars[i] == '[' {
                crates[index].push_back(chars[i + 1] as i32);
            }
        }
    }
    
    crates
}

fn part1(lines: Vec<String>) -> String {
    let mut parts = lines.split(|line| line.is_empty());

    let configuration = parts.next().unwrap();

    let mut crates = get_crates(configuration);    
    
    let instructions = parts.next().unwrap();
    
    for instruction in instructions {
        let mut parts = instruction.split_whitespace();
        let amount_to_move = parts.nth(1).unwrap().parse::<i32>().unwrap();
        let from = parts.nth(1).unwrap().parse::<i32>().unwrap() - 1;
        let to = parts.nth(1).unwrap().parse::<i32>().unwrap() - 1;

        for _ in 0..amount_to_move {
            let value = crates[from as usize].pop_front().unwrap();
            crates[to as usize].push_front(value);
        }
    }

    crates.iter().map(|c| *c.front().unwrap() as u8 as char).collect()
}

fn part2(lines: Vec<String>) -> i32 {
    0
}
