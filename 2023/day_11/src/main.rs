struct Reality {
    space: Vec<Vec<char>>,
    galaxies: Vec<(usize, usize)>,
}

fn main() {
    let raw = std::fs::read_to_string("input.txt").expect("could not read file");

    let content: Vec<&str> = raw.lines().filter(|line| !line.is_empty()).collect();

    let reality = get_space(content);

    let mut combinations = Vec::new();

    for i in 0..reality.galaxies.len() {
        for j in (i + 1)..reality.galaxies.len() {
            combinations.push((reality.galaxies[i], reality.galaxies[j]));
        }
    }

    let empty_rows = get_empty_rows(&reality);
    let empty_columns = get_empty_columns(&reality);

    let mut sum_p1 = 0;
    let mut sum_p2 = 0;

    for (start, end) in combinations {
        sum_p1 += get_shortest_path_manhattan(start, end, empty_rows.clone(), empty_columns.clone(), 1);
        sum_p2 += get_shortest_path_manhattan(start, end, empty_rows.clone(), empty_columns.clone(), 10);
    }

    println!("P1: {}", sum_p1);
    println!("P2: {}", sum_p2);
}

fn get_empty_rows(reality: &Reality) -> Vec<usize> {
    let mut empty_rows = Vec::new();

    for (index, row) in reality.space.iter().enumerate() {
        let collapsed_row: String = row.iter().collect();
        if collapsed_row.contains('#') {
            continue;
        }

        empty_rows.push(index);
    }

    empty_rows
}

fn get_empty_columns(reality: &Reality) -> Vec<usize> {
    let mut empty_columns = Vec::new();

    for column_index in 0..reality.space[0].len() {
        let mut collapsed_column: String = String::new();
        for j in 0..reality.space.len() {
            collapsed_column.push(reality.space[j][column_index]);
        }

        if collapsed_column.contains('#') {
            continue;
        }

        empty_columns.push(column_index);
    }

    empty_columns
}

fn get_space(content: Vec<&str>) -> Reality {
    let mut space: Vec<Vec<char>> = Vec::new();
    for line in content {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        space.push(row);
    }
    let mut reality = Reality { space, galaxies: Vec::new() };
    for (row_index, row) in reality.space.iter().enumerate() {
        for (column_index, c) in row.iter().enumerate() {
            if *c == '#' {
                reality.galaxies.push((row_index, column_index));
            }
        }
    }
    reality
}

fn get_shortest_path_manhattan(start: (usize, usize), end: (usize, usize), empty_rows: Vec<usize>, empty_columns: Vec<usize>, expansion_factor: usize) -> usize {
    let (start_row, start_column) = start;
    let (end_row, end_column) = end;

    let mut row_distance = if start_row > end_row {
        start_row - end_row
    } else {
        end_row - start_row
    };

    let mut column_distance = if start_column > end_column {
        start_column - end_column
    } else {
        end_column - start_column
    };

    for &empty_row in &empty_rows {
        if (start_row < end_row && empty_row > start_row && empty_row < end_row) ||
           (start_row > end_row && empty_row < start_row && empty_row > end_row) {
            row_distance += expansion_factor;
        }
    }

    for &empty_column in &empty_columns {
        if (start_column < end_column && empty_column > start_column && empty_column < end_column) ||
           (start_column > end_column && empty_column < start_column && empty_column > end_column) {
            column_distance += expansion_factor;
        }
    }

    row_distance + column_distance
}
