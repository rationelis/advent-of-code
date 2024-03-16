use utils::read_input_file;

struct Forest {
    trees: Vec<Vec<i32>>,
}

fn main() {
    let lines = read_input_file("input.txt").unwrap();

    let forest = parse_forest(lines.clone());

    let views = find_views_in_forest(forest);

    println!("Part 1: {}", views);
}

fn parse_forest(lines: Vec<String>) -> Forest {
    let trees = lines
        .into_iter()
        .map(|line| line.chars().map(|c| c as i32).collect())
        .collect();

    Forest { trees }
}

fn find_views_in_forest(forest: Forest) -> i32 {
    let mut views = 0;

    let rows = forest.trees.len();
    let cols = forest.trees[0].len();

    let circumference = (rows * 2) + (cols * 2) - 4;

    for row in 0..rows {
        for col in 0..cols {
            let current_value = forest.trees[row][col];
            let mut has_clear_view = true;

            for r in 0..row {
                if forest.trees[r][col] > current_value {
                    has_clear_view = false;
                    break;
                }
            }

            if has_clear_view {
                for r in row + 1..forest.trees.len() {
                    if forest.trees[r][col] > current_value {
                        has_clear_view = false;
                        break;
                    }
                }
            }

            if has_clear_view {
                for c in 0..col {
                    if forest.trees[row][c] > current_value {
                        has_clear_view = false;
                        break;
                    }
                }
            }

            if has_clear_view {
                for c in col + 1..forest.trees[row].len() {
                    if forest.trees[row][c] > current_value {
                        has_clear_view = false;
                        break;
                    }
                }
            }

            if has_clear_view {
                views += 1;
            }
        }
    }

   (views + circumference) as i32
}

