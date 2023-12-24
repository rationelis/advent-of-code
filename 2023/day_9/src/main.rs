use std::fs;

#[derive(Debug, Clone)]
struct History {
    parent: Option<Box<History>>,
    child: Option<Box<History>>,
    rows: Vec<i32>,
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("could not read file");

    let lines: Vec<&str> = content.lines().filter(|line| !line.is_empty()).collect();

    let mut histories: Vec<History> = get_history_from_lines(lines);

    for mut history in &mut histories {
        traverse_history(&mut history);
    }

    println!("p1: {}", solve_p1(histories.clone()));
    println!("p2: {}", solve_p2(histories.clone()));
}

fn solve_p1(histories: Vec<History>) -> i32 {
    let mut sum = 0;

    for history in histories {
        let mut history = history;

        while let Some(child) = history.child {
            history = *child;
        }

        let mut current_value = 0;

        while let Some(mut parent) = history.parent {
            let last_from_parent = parent.rows[parent.rows.len() - 1];
            let new_add = last_from_parent + current_value;
            parent.rows.push(new_add);
            current_value = new_add;
            history = *parent;
        }

        sum += current_value;
    }   

    sum
}

fn solve_p2(histories: Vec<History>) -> i32 {
    let mut sum = 0;

    for history in histories {
        let mut history = history;

        while let Some(child) = history.child {
            history = *child;
        }

        let mut current_value = 0;

        while let Some(mut parent) = history.parent {
            let first_from_row = parent.rows[0];
            let new_add = first_from_row - current_value;
            parent.rows.insert(0, new_add);
            current_value = new_add;
            history = *parent;
        }

        sum += current_value;
    }   

    sum
}

fn traverse_history(history: &mut History) {
    if history.rows.iter().all(|&x| x == 0) {
        return;
    }

    let mut differences = Vec::new();
    for i in 1..history.rows.len() {
        let difference = history.rows[i] - history.rows[i - 1];
        differences.push(difference);
    }

    let new_history = History {
        parent: Some(Box::new(history.clone())),
        child: None,
        rows: differences,
    };

    history.child = Some(Box::new(new_history));

    if let Some(child) = &mut history.child {
        traverse_history(child);
    }
}

fn get_history_from_lines(lines: Vec<&str>) -> Vec<History> {
    return lines
        .iter()
        .map(|line| {
            let mut history = History {
                parent: None,
                child: None,
                rows: Vec::new(),
            };
            for number in line.split(" ") {
                history.rows.push(number.parse::<i32>().unwrap());
            }
            history
        })
        .collect();
}
