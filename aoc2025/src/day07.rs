use std::collections::VecDeque;
use std::io::{stdin, BufRead};

const DIAGONAL_DOWN: [(isize, isize); 2] = [(1, -1), (1, 1)];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CellKind {
    Empty,
    Start,
    Splitter,
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    kind: CellKind,
}

impl Cell {
    fn new(kind: CellKind) -> Self {
        Cell { kind }
    }
}

struct Grid {
    data: Vec<Vec<Cell>>,
    start: (usize, usize),
    rows: usize,
    cols: usize,
}

impl Grid {
    fn from_lines(lines: &[String]) -> Self {
        let rows = lines.len();
        let cols = lines[0].len();

        let mut start_col = 0;
        let mut data = vec![vec![Cell::new(CellKind::Empty); cols]; rows];

        for (r, line) in lines.iter().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                match ch {
                    'S' => {
                        start_col = c;
                        data[r][c] = Cell::new(CellKind::Start)
                    }
                    '^' => data[r][c] = Cell::new(CellKind::Splitter),
                    _ => {}
                }
            }
        }

        Grid {
            data,
            start: (0, start_col),
            rows,
            cols,
        }
    }

    fn in_bounds(&self, r: isize, c: isize) -> bool {
        r >= 0 && r < self.rows as isize && c >= 0 && c < self.cols as isize
    }
}

fn reachable_splitters(grid: &Grid) -> Vec<(usize, usize)> {
    let mut reachable = Vec::new();
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; grid.cols]; grid.rows];

    queue.push_back(grid.start);
    visited[grid.start.0][grid.start.1] = true;

    while let Some((r, c)) = queue.pop_front() {
        if r + 1 >= grid.rows {
            continue;
        }

        let (br, bc) = (r + 1, c); // below
        let cell = grid.data[br][bc].kind;

        if cell == CellKind::Splitter {
            reachable.push((br, bc));

            // diagonals
            for &(dr, dc) in &DIAGONAL_DOWN {
                let nr = br as isize + dr;
                let nc = bc as isize + dc;
                if grid.in_bounds(nr, nc) {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if !visited[nr][nc] {
                        visited[nr][nc] = true;
                        queue.push_back((nr, nc));
                    }
                }
            }
        } else {
            // straight down
            if !visited[br][bc] {
                visited[br][bc] = true;
                queue.push_back((br, bc));
            }
        }
    }

    reachable
}

fn main() {
    let lines = stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    let grid = Grid::from_lines(&lines);
    let reachable = reachable_splitters(&grid);

    println!("Part 1: {}", reachable.len());
}
