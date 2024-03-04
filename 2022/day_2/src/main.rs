use utils::read_input_file;

fn main() {
    let lines = read_input_file("input.txt").unwrap();

    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}

fn get_points_part1(opponent: char, _move: char) -> i32 {
    match (opponent, _move) {
        ('A', 'X') => 1 + 3,
        ('A', 'Y') => 2 + 6,
        ('A', 'Z') => 3 + 0,
        ('B', 'X') => 1 + 0,
        ('B', 'Y') => 2 + 3,
        ('B', 'Z') => 3 + 6,
        ('C', 'X') => 1 + 6,
        ('C', 'Y') => 2 + 0,
        ('C', 'Z') => 3 + 3,
        _ => 0
    }
}

fn get_points_part2(opponent: char, _move: char) -> i32 {
    match (opponent, _move) {
        ('A', 'X') => 3 + 0,
        ('A', 'Y') => 1 + 3,
        ('A', 'Z') => 2 + 6,
        ('B', 'X') => 1 + 0,
        ('B', 'Y') => 2 + 3,
        ('B', 'Z') => 3 + 6,
        ('C', 'X') => 2 + 0,
        ('C', 'Y') => 3 + 3,
        ('C', 'Z') => 1 + 6,
        _ => 0
    }
}

fn part1(lines: Vec<String>) -> i32 {
    let mut score = 0;

    for line in lines {
        let round = line.split_whitespace().collect::<Vec<&str>>();
        let opponent = round[0].chars().nth(0).unwrap();  
        let _move = round[1].chars().nth(0).unwrap();
        score += get_points_part1(opponent, _move);
    }

    score
}

fn part2(lines: Vec<String>) -> i32 {
    let mut score = 0;

    for line in lines {
        let round = line.split_whitespace().collect::<Vec<&str>>();
        let opponent = round[0].chars().nth(0).unwrap();  
        let _move = round[1].chars().nth(0).unwrap();
        score += get_points_part2(opponent, _move);
    }

    score
}

