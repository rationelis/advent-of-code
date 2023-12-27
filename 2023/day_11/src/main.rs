struct Reality {
    space: Vec<Vec<char>>,
    galaxies: Vec<(usize, usize)>,
}

fn main() {
    let raw = std::fs::read_to_string("input.txt").expect("could not read file");

    let content: Vec<&str> = raw.lines().filter(|line| !line.is_empty()).collect();

    let mut reality = get_space(content);

    expand_space(&mut reality);

    // for row in &reality.space {
    //     for c in row {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    locate_galaxies(&mut reality);

    // println!("{:?}", reality.galaxies);

    let mut combinations = Vec::new();

    for i in 0..reality.galaxies.len() {
        for j in (i + 1)..reality.galaxies.len() {
            combinations.push((reality.galaxies[i], reality.galaxies[j]));
        }
    }

    // println!("{:?}", combinations.len());
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
    Reality {
        space,
        galaxies: Vec::new(),
    }
}

fn expand_space(reality: &mut Reality) -> &Reality {
    let mut rows_to_insert = Vec::new();
    let mut columns_to_insert = Vec::new();

    for (index, row) in reality.space.iter().enumerate() {
        let collapsed_row: String = row.iter().collect();
        if collapsed_row.contains('#') {
            continue;
        }

        rows_to_insert.push(index);
    }

    for index in rows_to_insert {
        reality
            .space
            .insert(index, vec!['.'; reality.space[index].len()]);
    }

    for column_index in 0..reality.space[0].len() {
        let mut collapsed_column: String = String::new();
        for j in 0..reality.space.len() {
            collapsed_column.push(reality.space[j][column_index]);
        }

        if collapsed_column.contains('#') {
            continue;
        }

        columns_to_insert.push(column_index);
    }

    for column_index in columns_to_insert {
        for row in &mut reality.space {
            row.insert(column_index, '.');
        }
    }

    reality
}

fn locate_galaxies(reality: &mut Reality) -> &Reality {
    for (row_index, row) in reality.space.iter().enumerate() {
        for (column_index, c) in row.iter().enumerate() {
            if *c == '#' {
                reality.galaxies.push((row_index, column_index));
            }
        }
    }

    reality
}
