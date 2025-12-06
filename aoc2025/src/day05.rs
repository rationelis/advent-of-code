use std::io::{stdin, BufRead};

fn main() {
    let (mut ranges, values) = parse_input();

    ranges.sort_unstable_by_key(|r| r.0);

    // Part 1
    let mut count_p1 = 0;
    for value in values {
        for (start, end) in &ranges {
            if value >= *start && value <= *end {
                count_p1 += 1;
                break;
            }
        }
    }

    println!("Part 1: {}", count_p1);

    // Part 2
    let mut count_p2 = 0;

    let mut curr_start = ranges[0].0;
    let mut curr_end = ranges[0].1;

    for (start, end) in &ranges[1..] {
        if *start > curr_end + 1 {
            // Disjoint range
            count_p2 += curr_end - curr_start + 1;
            curr_start = *start;
            curr_end = *end;
        } else {
            // Overlapping range
            curr_end = curr_end.max(*end);
        }
    }

    count_p2 += curr_end - curr_start + 1;

    println!("Part 2: {}", count_p2);
}

fn parse_input() -> (Vec<(i64, i64)>, Vec<i64>) {
    let std = stdin();
    let lines = std
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    let split_index = lines
        .iter()
        .position(|l| l.trim().is_empty())
        .unwrap_or(lines.len());

    let (ranges, values) = lines.split_at(split_index);

    let ranges = ranges
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            let start = parts[0].parse::<i64>().unwrap();
            let end = parts[1].parse::<i64>().unwrap();
            (start, end)
        })
        .collect::<Vec<(i64, i64)>>();

    let values = values
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    (ranges, values)
}
