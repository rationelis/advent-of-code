use utils::read_input_file;
use std::collections::HashSet;

fn main() {
    let lines = read_input_file("input.txt").unwrap();

    let part1 = find_first_dinstinct_subset(lines[0].clone(), 4).unwrap();
    let part2 = find_first_dinstinct_subset(lines[0].clone(), 14).unwrap();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn find_first_dinstinct_subset(input: String, length: usize) -> Option<i32> {
    let mut start = 0;
    let mut end = length;

    for _ in 0..input.len() - length {
        let subset = &input[start..end];
        
        let mut set = HashSet::new();
        for c in subset.chars() {
            set.insert(c);
        }

        if set.len() == length {
            return Some(end as i32);
        }

        start += 1;
        end += 1;
    }

    None
}
