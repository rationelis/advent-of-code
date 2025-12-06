use utils::read_input_file;
use std::collections::HashSet;

struct Knot {
    position: (i32, i32),
}

impl Knot {
    fn move_to(&self, direction: (i32, i32)) -> Knot {
        Knot { position: (self.position.0 + direction.0, self.position.1 + direction.1) }
    }
}

#[derive(Clone)]
struct Instruction {
    direction: (i32, i32),
    steps: i32,
}

fn parse_direction(direction: char) -> (i32, i32) {
    match direction {
        'U' => (0, 1),
        'D' => (0, -1),
        'L' => (-1, 0),
        'R' => (1, 0),
        _ => (0, 0),
    }
}

fn main() {
    let instructions_file = read_input_file("input.txt").unwrap();

    let instructions = parse_instructions(instructions_file);

    let part1 = solve_for_n_knots(instructions.clone(), 2);
    let part2 = solve_for_n_knots(instructions.clone(), 10);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_for_n_knots(instructions: Vec<Instruction>, n: usize) -> usize {
    let mut knots = Vec::new();

    for _ in 0..n {
        knots.push(Knot { position: (0, 0) });
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((0, 0));

    for instruction in instructions {
        for _ in 0..instruction.steps {
            knots[0] = knots[0].move_to(instruction.direction);

            for i in 1..n {
                let diff_x = knots[i - 1].position.0 - knots[i].position.0;
                let diff_y = knots[i - 1].position.1 - knots[i].position.1;

                if diff_x.abs() > 1 || diff_y.abs() > 1 {
                    knots[i] = knots[i].move_to((diff_x.signum(), diff_y.signum()));
                }

                visited.insert((knots[n - 1].position.0 as usize, knots[n - 1].position.1 as usize));
            }
        }
    }
    
    visited.len()
}

fn parse_instructions(lines: Vec<String>) -> Vec<Instruction> {
    lines.iter().map(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let direction = parse_direction(parts[0].chars().next().unwrap());
        let steps = parts[1].parse::<i32>().unwrap();
        Instruction { direction, steps }
    }).collect()
}

