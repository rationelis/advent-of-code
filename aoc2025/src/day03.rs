use std::io::{self, BufRead};

fn main() {
    let mut p1_total: u128 = 0;
    let mut p2_total: u128 = 0;

    for line in io::stdin().lock().lines() {
        let s = line.unwrap();
        let s = s.trim();
        if s.is_empty() {
            continue;
        }

        let bytes = s.as_bytes();

        p1_total += sub(bytes, 2);
        p2_total += sub(bytes, 12);
    }

    println!("Part 1: {}", p1_total);
    println!("Part 2: {}", p2_total);
}

fn sub(bytes: &[u8], k: usize) -> u128 {
    let n = bytes.len();
    let mut result = 0u128;
    let mut start = 0;

    for remaining in (1..=k).rev() {
        let end = n - remaining;
        let mut best = b'0';
        let mut best_i = start;

        for (i, &c) in bytes.iter().enumerate().skip(start).take(end - start + 1) {
            if c > best {
                best = c;
                best_i = i;
                if c == b'9' {
                    break;
                }
            }
        }

        result = result * 10 + (best - b'0') as u128;
        start = best_i + 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(sub("987654321111111".as_bytes(), 2), 98);
        assert_eq!(sub("811111111111119".as_bytes(), 2), 89);
        assert_eq!(sub("234234234234278".as_bytes(), 2), 78);
        assert_eq!(sub("818181911112111".as_bytes(), 2), 92);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(sub("987654321111111".as_bytes(), 12), 987654321111);
        assert_eq!(sub("811111111111119".as_bytes(), 12), 811111111119);
        assert_eq!(sub("234234234234278".as_bytes(), 12), 434234234278);
        assert_eq!(sub("818181911112111".as_bytes(), 12), 888911112111);
    }
}
