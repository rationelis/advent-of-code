use utils::read_input_file;
use std::collections::HashSet;

struct Knot {
    position: (i32, i32),
}

impl Knot {
    fn move_to(&self, direction: (i32, i32), bridge: &Bridge) -> Knot {
        if self.position.0 + direction.0 < 0 || self.position.0 + direction.0 >= bridge.width as i32 {
            return Knot { position: self.position };
        }

        if self.position.1 + direction.1 < 0 || self.position.1 + direction.1 >= bridge.height as i32 {
            return Knot { position: self.position };
        }

        Knot { position: (self.position.0 + direction.0, self.position.1 + direction.1) }
    }

    fn find_manhattan_distance(&self, other: &Knot, direction: (i32, i32), bridge: &Bridge) -> i32 {
        let moved_tail = other.move_to(direction, &bridge);
        let distance = (moved_tail.position.0 - self.position.0).abs() + (moved_tail.position.1 - self.position.1).abs();
        distance
    }
}

struct Bridge { 
    rope_area: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

struct Instruction {
    direction: (i32, i32),
    steps: i32,
}

fn parse_direction(direction: char) -> (i32, i32) {
    match direction {
        'U' => (0, -1),
        'D' => (0, 1),
        'L' => (-1, 0),
        'R' => (1, 0),
        _ => (0, 0),
    }
}

fn main() {
    let area_file = read_input_file("area.txt").unwrap();
    
    let bridge = parse_area(area_file);

    let instructions_file = read_input_file("input.txt").unwrap();

    let instructions = parse_instructions(instructions_file);

    let mut head = Knot { position: (0, bridge.height as i32 - 1) };
    let mut tail = Knot { position: (0, bridge.height as i32 - 1) };

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    
    let all_directions = vec![(0, -1), (0, 1), (-1, 0), (1, 0), (-1, -1), (1, -1), (-1, 1), (1, 1)];

    for instruction in instructions {
        for _ in 0..instruction.steps {
            head = head.move_to(instruction.direction, &bridge);
            
            let mut min_distance = std::i32::MAX;
            let mut min_direction = (0, 0);

            for direction in &all_directions {
                let distance = tail.find_manhattan_distance(&head, *direction, &bridge);
                if distance < min_distance {
                    min_distance = distance;
                    min_direction = *direction;
                }
            }

            println!("Mindistance: {}", min_distance);
            println!("Min direction: {}, {}", min_direction.0, min_direction.1);

            tail = tail.move_to(min_direction, &bridge);

            visited.insert((tail.position.0 as usize, tail.position.1 as usize));

            print_area_with_knots(&bridge, &head, &tail);
        }
    }
}

fn parse_area(lines: Vec<String>) -> Bridge {
    let area: Vec<Vec<char>> = lines.iter().map(|line| {
        line.chars().collect()
    }).collect();
    Bridge { 
        rope_area: area.clone(),
        width: area[0].len(),
        height: area.len(),
    }
}

fn parse_instructions(lines: Vec<String>) -> Vec<Instruction> {
    lines.iter().map(|line| {
        let chars = line.chars().collect::<Vec<char>>();        
        let direction = parse_direction(chars[0]);
        let steps = chars[2].to_string().parse::<i32>().unwrap();
        Instruction { direction, steps }
    }).collect()
}

fn print_area_with_knots(bridge: &Bridge, head: &Knot, tail: &Knot) {
    for y in 0..bridge.height {
        for x in 0..bridge.width {
            if head.position.0 == x as i32 && head.position.1 == y as i32 {
                print!("H");
            } else if tail.position.0 == x as i32 && tail.position.1 == y as i32 {
                print!("T");
            } else {
                print!("{}", bridge.rope_area[y][x]);
            }
        }
        println!();
    }
}

