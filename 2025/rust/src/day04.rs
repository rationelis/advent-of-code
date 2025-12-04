use std::io::{stdin, BufRead};

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

struct Grid {
    data: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn empty(rows: usize, cols: usize) -> Self {
        let data = vec![vec![Cell::new(CellKind::Empty); cols]; rows];
        Grid { data, rows, cols }
    }

    fn set_obstacles(&mut self, targets: &[(usize, usize)]) {
        for (r, c) in targets {
            self.data[*r][*c] = Cell::new(CellKind::Obstacle);
        }
    }

    fn adjacent_obstacles(&self, r: usize, c: usize) -> usize {
        DIRECTIONS
            .iter()
            .filter(|(dr, dc)| {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                self.data
                    .get(nr as usize)
                    .and_then(|row| row.get(nc as usize))
                    .is_some_and(|cell| cell.is_obstacle())
            })
            .count()
    }

    fn count_accessible_obstacles(&mut self, and_repeat: bool) -> usize {
        let mut count = 0;

        loop {
            let mut accessible = Vec::new();

            for r in 0..self.rows {
                for c in 0..self.cols {
                    if self.data[r][c].is_accessible(self, r, c) {
                        accessible.push((r, c));
                    }
                }
            }

            if accessible.is_empty() {
                break;
            }

            count += accessible.len();

            for (r, c) in &accessible {
                self.data[*r][*c] = Cell::new(CellKind::Target);
            }

            if !and_repeat {
                break;
            }
        }

        count
    }
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    kind: CellKind,
}

#[derive(Debug, Clone, Copy)]
enum CellKind {
    Empty,
    Obstacle,
    Target,
}

impl Cell {
    fn new(kind: CellKind) -> Self {
        Cell { kind }
    }

    fn is_accessible(&self, grid: &Grid, r: usize, c: usize) -> bool {
        self.is_obstacle() && grid.adjacent_obstacles(r, c) < 4
    }

    fn is_obstacle(&self) -> bool {
        matches!(self.kind, CellKind::Obstacle)
    }
}

fn main() {
    let (n, targets) = parse_input();

    let mut grid_p1 = Grid::empty(n, n);
    grid_p1.set_obstacles(&targets);
    println!("Part 1: {}", grid_p1.count_accessible_obstacles(false));

    let mut grid_p2 = Grid::empty(n, n);
    grid_p2.set_obstacles(&targets);
    println!("Part 2: {}", grid_p2.count_accessible_obstacles(true));
}

fn parse_input() -> (usize, Vec<(usize, usize)>) {
    let stdin = stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    let n = lines.len();
    let mut targets = Vec::new();
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '@' {
                targets.push((r, c));
            }
        }
    }
    (n, targets)
}
