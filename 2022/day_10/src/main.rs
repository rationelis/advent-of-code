use utils::read_input_file;

fn main() {
    let lines = read_input_file("input.txt").unwrap();

    let part1 = handle_instructions(lines.clone());

    println!("Part 1: {}", part1);
}

fn handle_instructions(lines: Vec<String>) -> i32 {
    let mut register_value = 1;
    let mut cycle_count = 1;

    let mut signal_strengths = Vec::new();
    
    for line in lines {
        let mut parts = line.split_whitespace();

        let instruction = parts.next().unwrap();

        match instruction {
            "noop" => {
                cycle_count += 1;
                if is_part1_cycle(cycle_count) {
                    signal_strengths.push(cycle_count * register_value);
                }
                handle_pixel(cycle_count, register_value);
            },
            "addx" => {
                let value = parts.next().unwrap().parse::<i32>().unwrap();

                cycle_count += 1;
                if is_part1_cycle(cycle_count) {
                    signal_strengths.push(cycle_count * register_value);
                }
                handle_pixel(cycle_count, register_value);
                
                cycle_count += 1;
                register_value += value;
                if is_part1_cycle(cycle_count) {
                    signal_strengths.push(cycle_count * register_value);
                }
                handle_pixel(cycle_count, register_value);
            },
            _ => {
                println!("Unknown instruction: {}", instruction);
            }
        }
    }

    signal_strengths.iter().sum::<i32>()
}

fn handle_pixel(cycle_count: i32, register_value: i32) {
    if is_pixel_in_drawing_distance(cycle_count - 1, register_value) {
        print!("#");
    } else {
        print!(".");
    }
    if is_part2_cycle(cycle_count - 1) {
        println!("");
    }
}

fn is_part1_cycle(cycle_count: i32) -> bool {
    if cycle_count > 220 {
        return false;
    }
    (cycle_count - 20) % 40 == 0
}

fn is_part2_cycle(cycle_count: i32) -> bool {
    if cycle_count > 240 {
        return false;
    }
    cycle_count % 40 == 0
}

fn is_pixel_in_drawing_distance(cycle_count: i32, register_value: i32) -> bool {
    ((cycle_count % 40) - 1 == register_value) || ((cycle_count % 40) == register_value) || ((cycle_count % 40) + 1 == register_value)
}

