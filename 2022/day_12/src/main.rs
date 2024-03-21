use utils::read_input_file;
use std::collections::{HashSet, VecDeque};

struct Grid {
    grid: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Grid {
    fn look_around(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    
        directions.iter().filter_map(|(i, j)| {
            let new_x = x as i32 + i;
            let new_y = y as i32 + j;

            if new_x >= 0 && new_x < self.grid.len() as i32 && new_y >= 0 && new_y < self.grid[0].len() as i32 {
                Some((new_x as usize, new_y as usize))
            } else {
                None
            }
        }).collect()
    }

    fn is_steppable(&self, (x, y): (usize, usize), (i, j): (usize, usize)) -> bool {
        let from = self.grid[x][y] as i32;
        let to = self.grid[i][j] as i32;

        let diff = to - from;

        diff <= 1
    }

    fn find_lowest_elevation(&self) -> Vec<(usize, usize)> {
        self.grid.iter().enumerate().flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &c)| {
                if c == 'a' {
                    Some((i, j))
                } else {
                    None
                }
            })
        }).collect()
    }
}

fn main() {
    let lines = read_input_file("input.txt").unwrap();
    
    let grid = get_grid(lines);

    let part1 = find_shortest_path(&grid);

    println!("Part 1: {}", part1);

    let lowest_elevation = grid.find_lowest_elevation(); 

    let part2 = lowest_elevation.iter().map(|&(x, y)| {
        let mut new_grid = grid.grid.clone();
        new_grid[x][y] = 'a';

        let new_grid = Grid { grid: new_grid, start: (x, y), end: grid.end };

        find_shortest_path(&new_grid)
    });

    let part2 = part2.filter(|&x| x != -1).min().unwrap();

    println!("Part 2: {}", part2);
}

fn find_shortest_path(grid: &Grid) -> i32 {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue = VecDeque::new();

    let mut distance_map: Vec<Vec<usize>> = vec![vec![0; grid.grid[0].len()]; grid.grid.len()];

    queue.push_back((grid.start, 0));

    while let Some(((x, y), distance)) = queue.pop_front() {
        if (x, y) == grid.end {
            return distance as i32;
        }

        visited.insert((x, y));

        let neighbors = grid.look_around(x, y);

        for (i, j) in neighbors {
            if grid.is_steppable((x, y), (i, j)) && !visited.contains(&(i, j)) {
                queue.push_back(((i, j), distance + 1));
                visited.insert((i, j));
                distance_map[i][j] = distance_map[x][y] + 1;
            }
        }
    }
    
    -1
}

fn get_grid(lines: Vec<String>) -> Grid {
    let mut grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| c == 'S').map(|j| (i, j)))
        .unwrap();

    grid[start.0][start.1] = 'a';

    let end = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| c == 'E').map(|j| (i, j)))
        .unwrap();

    grid[end.0][end.1] = 'z';

    Grid { grid, start, end }
}
