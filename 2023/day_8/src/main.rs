use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let content = fs::read_to_string("input.txt").expect("could not read file");

    let lines: Vec<&str> = content.lines().filter(|line| !line.is_empty()).collect();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in lines.iter().skip(1) {
        if let Some((key, rest)) = line.split_once(" = ") {
            let (left, right) = rest
                .trim_matches(|c| c == '(' || c == ')')
                .split_once(',')
                .unwrap_or(("", ""));
            let tuple = (left.trim(), right.trim());
            map.insert(key, tuple);
        }
    }

    let directions: Vec<char> = lines[0].chars().collect();

    println!("Part 1: {}", solve_p1(&map.clone(), &directions.clone()));
    let start = Instant::now();
    println!("Part 2: {}", solve_p2(&map.clone(), &directions.clone()));
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

fn solve_p1(map: &HashMap<&str, (&str, &str)>, directions: &Vec<char>) -> usize {
    let mut location = "AAA";
    let mut steps = 0;

    while location != "ZZZ" {
        for direction in directions.iter() {
            let (left, right) = map.get(location).unwrap();
            if *direction == 'L' {
                location = left;
            } else {
                location = right;
            }
            steps += 1;
        }
    }

    steps
}

fn solve_p2(map: &HashMap<&str, (&str, &str)>, directions: &Vec<char>) -> usize {
    let mut path_length: Vec<usize> = Vec::new();

    let starting_nodes = map
        .iter()
        .filter(|(key, _)| key.ends_with("A"))
        .map(|(key, _)| key)
        .collect::<Vec<_>>();

    for node in starting_nodes {
        let mut location = node;
        let mut steps = 0;

        while !location.ends_with("Z") {
            for direction in directions.iter() {
                let (left, right) = map.get(location).unwrap();
                if *direction == 'L' {
                    location = left;
                } else {
                    location = right;
                }
                steps += 1;
            }
        }    

        path_length.push(steps);    
    }

    let mut lcm = path_length[0];

    for i in 1..path_length.len() {
        lcm = lcm * path_length[i] / gcd(lcm, path_length[i]);
    }

    lcm
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}