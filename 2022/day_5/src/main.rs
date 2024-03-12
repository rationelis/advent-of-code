use utils::read_input_file;
use std::collections::VecDeque;

#[derive(Clone)]
struct Crate {
    stack: VecDeque<i32>,
}

#[derive(Clone, Copy)]
struct MoveInstruction {
    amount_to_move: i32,
    from: i32,
    to: i32,
}

fn main() {
    let lines = read_input_file("input.txt").unwrap();

    let mut parts = lines.split(|line| line.is_empty());

    let configuration = parts.next().unwrap();

    let crates = get_crates(configuration);

    let instructions = parts.next().unwrap();

    let moves = parse_instructions(instructions);

    let mut crates_part1 = crates.clone();
    let mut crates_part2 = crates.clone();

    for instruction in moves.iter() {
        perform_move(&mut crates_part1, *instruction);
        perform_move_reverse(&mut crates_part2, *instruction);
    }

    let part1 = crates_part1.iter().map(|c| *c.stack.front().unwrap() as u8 as char).collect::<String>();
    let part2 = crates_part2.iter().map(|c| *c.stack.front().unwrap() as u8 as char).collect::<String>();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn parse_instructions(instructions: &[String]) -> Vec<MoveInstruction> {
    let mut move_instructions = Vec::new();

    for line in instructions {
        let mut parts = line.split_whitespace();
        let amount_to_move = parts.nth(1).unwrap().parse::<i32>().unwrap();
        let from = parts.nth(1).unwrap().parse::<i32>().unwrap() - 1;
        let to = parts.nth(1).unwrap().parse::<i32>().unwrap() - 1;

        move_instructions.push(MoveInstruction {
            amount_to_move,
            from,
            to,
        });
    }

    move_instructions
}

fn perform_move(crates: &mut Vec<Crate>, instruction: MoveInstruction) {
    for _ in 0..instruction.amount_to_move {
        let value = crates[instruction.from as usize].stack.pop_front().unwrap();
        crates[instruction.to as usize].stack.push_front(value);
    }
}

fn perform_move_reverse(crates: &mut Vec<Crate>, instruction: MoveInstruction) {
    let mut values = Vec::new();

    for _ in 0..instruction.amount_to_move {
        values.push(crates[instruction.from as usize].stack.pop_front().unwrap());
    }

    values.reverse();

    for value in values {
        crates[instruction.to as usize].stack.push_front(value);
    }
}

fn get_crates(configuration: &[String]) -> Vec<Crate> {
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
    
    crates.iter().map(|stack| Crate { stack: stack.clone() }).collect()     
}

