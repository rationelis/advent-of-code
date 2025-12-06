use std::io::BufRead;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Multiply,
}

impl TryFrom<char> for Operation {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '+' => Ok(Operation::Add),
            '*' => Ok(Operation::Multiply),
            _ => Err(format!("Unsupported operation: {}", c)),
        }
    }
}

#[derive(Debug)]
struct Problem {
    values: Vec<i64>,
    operation: Operation,
}

impl Problem {
    fn new(operation: Operation) -> Self {
        Problem {
            values: Vec::new(),
            operation,
        }
    }

    fn solve(&self) -> i64 {
        match self.operation {
            Operation::Add => self.values.iter().sum(),
            Operation::Multiply => self.values.iter().product(),
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .map_while(|line| line.ok())
        .collect::<Vec<String>>();

    let (operation_lines, value_lines) = lines.split_last().unwrap();

    let operations: Vec<Operation> = operation_lines
        .chars()
        .filter(|&c| !c.is_whitespace())
        .map(|c| Operation::try_from(c).unwrap())
        .collect();

    let mut problems_p1: Vec<Problem> = operations.iter().copied().map(Problem::new).collect();
    parse_input_p1(&mut problems_p1, value_lines);
    println!(
        "Part 1: {}",
        problems_p1.iter().map(|p| p.solve()).sum::<i64>()
    );

    let mut problems_p2: Vec<Problem> = operations.iter().copied().map(Problem::new).collect();
    parse_input_p2(&mut problems_p2, value_lines);
    println!(
        "Part 2: {}",
        problems_p2.iter().map(|p| p.solve()).sum::<i64>()
    );
}

fn parse_input_p1(problems: &mut [Problem], value_lines: &[String]) {
    for line in value_lines {
        for (p, token) in problems.iter_mut().zip(line.split_whitespace()) {
            p.values.push(token.parse().unwrap());
        }
    }
}

fn parse_input_p2(problems: &mut [Problem], value_lines: &[String]) {
    let grid: Vec<Vec<char>> = value_lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let cols = grid[0].len();

    let separators: Vec<_> = (0..cols)
        .filter(|&col| grid.iter().all(|row| row[col].is_whitespace()))
        .collect();

    let sep_set: std::collections::HashSet<_> = separators.iter().cloned().collect();

    let mut col_acc = Vec::<char>::new();
    let mut problem_index = 0;

    for col in 0..cols {
        if sep_set.contains(&col) {
            if !col_acc.is_empty() {
                col_acc.clear();
            }
            problem_index += 1;
            continue;
        }
        for row in &grid {
            let c = row[col];
            if !c.is_whitespace() {
                col_acc.push(c);
            }
        }
        if let Ok(v) = col_acc.iter().collect::<String>().parse::<i64>() {
            problems[problem_index].values.push(v);
        }
        col_acc.clear();
    }
}
