use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut dial: i32 = 50;
    let mut zero_count_p1 = 0;
    let mut zero_count_p2 = 0;

    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() {
            continue;
        }
        let (dir, steps) = l.split_at(1);
        let steps: i32 = steps.parse().unwrap();

        let delta = match dir {
            "L" => -1,
            "R" => 1,
            _ => 0,
        };

        for _ in 0..steps {
            dial = (dial + delta).rem_euclid(100);
            if dial == 0 {
                zero_count_p2 += 1;
            }
        }

        if dial == 0 {
            zero_count_p1 += 1;
        }
    }

    println!("Part 1: {}", zero_count_p1);
    println!("Part 2: {}", zero_count_p2);
}
