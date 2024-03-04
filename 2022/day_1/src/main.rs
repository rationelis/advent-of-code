use utils::read_input_file;

fn main() {
    let lines = read_input_file("input.txt").unwrap();

    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}

fn part1(lines: Vec<String>) -> i32 {
    let groups = lines.split(|line| line.is_empty());

    let mut largest = 0;

    for group in groups {
        let mut group_sum = 0;
        for entry in group {
            let parse = entry.parse::<i32>().unwrap();
            group_sum += parse;
        }
        if group_sum > largest {
            largest = group_sum;
        }
    }
    
    largest
}

fn part2(lines: Vec<String>) -> i32 {
   let groups = lines.split(|line| line.is_empty());

   let mut sums = Vec::new();

   for group in groups {
       let mut group_sum = 0;
       for entry in group {
           let parse = entry.parse::<i32>().unwrap();
           group_sum += parse;
       }
       sums.push(group_sum);
   }

   sums.sort();

   sums[sums.len() - 1] + sums[sums.len() - 2] + sums[sums.len() - 3]
}
