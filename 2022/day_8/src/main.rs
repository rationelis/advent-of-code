use utils::read_input_file;

struct Forest {
    trees: Vec<Vec<i32>>,
}

fn main() {
    let lines = read_input_file("input.txt").unwrap();

    let forest = parse_forest(lines.clone());

    let views = count_visible_trees(&forest);

    print_forest(&forest);

    println!("Total visible trees: {}", views);
}

fn print_forest(forest: &Forest) {
    for row in 0..forest.trees.len() {
        for col in 0..forest.trees[0].len() {
            print!("{}", forest.trees[row][col]);
        }
        println!();
    }
}

fn parse_forest(lines: Vec<String>) -> Forest {
    let trees = lines
        .into_iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    Forest { trees }
}

fn is_tree_visible_in_horizontal_range(forest: &Forest, start_col: usize, end_col: usize, row: usize, value: i32) -> bool {
    for col in start_col..end_col {
        if forest.trees[row][col] > value {
            return false;
        }
    }

    true
}

fn is_tree_visible_in_vertical_range(forest: &Forest, start_row: usize, end_row: usize, col: usize, value: i32) -> bool {
    for row in start_row..end_row {
        if forest.trees[row][col] > value {
            return false;
        }
    }

    true 
}

fn count_visible_trees(forest: &Forest) -> i32 {
    let mut visible_trees = 0;

    for row in 0..forest.trees.len() {
        for col in 0..forest.trees[0].len() {
            let value = forest.trees[row][col];
            if is_tree_visible_in_horizontal_range(forest, 0, col, row, value) ||
               is_tree_visible_in_horizontal_range(forest, col, forest.trees[0].len(), row, value) ||
               is_tree_visible_in_vertical_range(forest, 0, row, col, value) ||
               is_tree_visible_in_vertical_range(forest, row, forest.trees.len(), col, value) {
                visible_trees += 1;
                println!("Tree at ({}, {}) is visible", row, col);
            }
        }
    }

    visible_trees
}

