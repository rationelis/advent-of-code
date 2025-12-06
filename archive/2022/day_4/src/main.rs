use utils::read_input_file;
use std::collections::HashSet;

fn main() {
    let lines = read_input_file("input.txt").unwrap();

    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}

fn is_either_subset(set_a: &HashSet<i32>, set_b: &HashSet<i32>) -> bool {
    set_a.is_subset(set_b) || set_b.is_subset(set_a)
}

fn is_disjoint(set_a: &HashSet<i32>, set_b: &HashSet<i32>) -> bool {
    set_a.is_disjoint(set_b)
} 

fn parse_range(line: &str) -> Vec<HashSet<i32>> {
    line.split(",").map(|section_range| {
        let mut parts = section_range.split("-");
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        (start..=end).collect()
    }).collect()
}

fn part1(lines: Vec<String>) -> i32 {
    lines.iter().map(|line| {
        let ranges = parse_range(line);
        if is_either_subset(&ranges[0], &ranges[1]) {
            1
        } else {
            0
        }
    }).sum()
}

fn part2(lines: Vec<String>) -> i32 {
    lines.iter().map(|line| {
        let ranges = parse_range(line);
        if is_disjoint(&ranges[0], &ranges[1]) {
            0
        } else {
            1
        }
    }).sum()
}
