use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let (mut p1, mut p2) = (0i64, 0i64);

    for range in line.split(',') {
        let mut parts = range.split('-');
        let start: i64 = parts.next().unwrap().parse().unwrap();
        let end: i64 = parts.next().unwrap().parse().unwrap();

        for n in start..=end {
            let s = n.to_string();
            let len = s.len();

            // Part 1: exactly 2 repetitions (split in half)
            if len % 2 == 0 && s[..len / 2] == s[len / 2..] {
                p1 += n;
            }

            // Part 2: at least 2 repetitions (any divisor)
            for chunk in 1..len {
                if len % chunk == 0
                    && s.as_bytes()
                        .chunks(chunk)
                        .all(|c| c == &s.as_bytes()[..chunk])
                {
                    p2 += n;
                    break;
                }
            }
        }
    }

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
