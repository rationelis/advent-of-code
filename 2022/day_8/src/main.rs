use utils::read_input_file;

struct Forest {
    trees: Vec<Vec<i32>>,
}

fn main() {
    let lines = read_input_file("input.txt").unwrap();

    let forest = parse_forest(lines.clone());

    let visible_trees = solve_part1(&forest);

    println!("Part 1: {}", visible_trees);
    
    let max_scenic_score = solve_part2(&forest);

    println!("Part 2: {}", max_scenic_score)
}

fn parse_forest(lines: Vec<String>) -> Forest {
    let trees = lines
        .into_iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    Forest { trees }
}

fn is_visible_horizontal(forest: &Forest, row: usize, start: usize, end: usize, value: i32) -> bool {
    for col in start..end {
        if forest.trees[row][col] >= value { 
            return false;
        }
    }

    true
}

fn is_visible_vertical(forest: &Forest, col: usize, start: usize, end: usize, value: i32) -> bool {
    for row in start..end {
        if forest.trees[row][col] >= value { 
            return false;
        }
    }

    true
}

fn find_viewing_distance_horizontal(forest: &Forest, row: usize, start: usize, end: usize, value: i32, reverse: bool) -> i32 {
    let mut distance = 0;

    if reverse {
        for col in (start..end).rev() {
            if forest.trees[row][col] >= value { 
                return distance + 1;
            }

            distance += 1;
        }
    } else {
        for col in start..end {
            if forest.trees[row][col] >= value { 
                return distance + 1;
            }

            distance += 1;
        }
    } 

    distance
}

fn find_viewing_distance_vertical(forest: &Forest, col: usize, start: usize, end: usize, value: i32, reverse: bool) -> i32 {
    let mut distance = 0;

    if reverse {
        for row in (start..end).rev() {
            if forest.trees[row][col] >= value { 
                return distance + 1;
            }

            distance += 1;
        }
    } else {
        for row in start..end {
            if forest.trees[row][col] >= value { 
                return distance + 1;
            }

            distance += 1;
        }
    }

    distance
}

fn solve_part1(forest: &Forest) -> i32 {
    let mut visible_trees = 0;

    for row in 0..forest.trees.len() {
        for col in 0..forest.trees[0].len() {
            let current = forest.trees[row][col];

            let is_visible_from_left = is_visible_horizontal(forest, row, 0, col, current);
            let is_visible_from_right = is_visible_horizontal(forest, row, col + 1, forest.trees[0].len(), current);
            
            let is_visible_from_top = is_visible_vertical(forest, col, 0, row, current);
            let is_visible_from_bottom = is_visible_vertical(forest, col, row + 1, forest.trees.len(), current);
            
            if is_visible_from_left || is_visible_from_right || is_visible_from_top || is_visible_from_bottom {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

fn solve_part2(forest: &Forest) -> i32 {
    let mut scores = Vec::new(); 

    for row in 0..forest.trees.len() {
        for col in 0..forest.trees[0].len() {
            let current = forest.trees[row][col];

            let visible_from_left = find_viewing_distance_horizontal(forest, row, 0, col, current, true);
            let visible_from_right = find_viewing_distance_horizontal(forest, row, col + 1, forest.trees[0].len(), current, false);

            let visible_from_top = find_viewing_distance_vertical(forest, col, 0, row, current, true);
            let visible_from_bottom = find_viewing_distance_vertical(forest, col, row + 1, forest.trees.len(), current, false);

            let score = visible_from_left * visible_from_right * visible_from_top * visible_from_bottom;
            scores.push(score);
        }
    }

    scores.iter().max().unwrap().clone()
}
