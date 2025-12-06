#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    pipe: Pipe,
}

#[derive(Debug, Clone)]
struct Game {
    maze: Vec<Vec<Point>>,
    starting_point: Point,
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pipe {
    VERTICAL,
    HORIZONTAL,
    L,
    J,
    SEVEN,
    F,
    GROUND,
    START,
}

impl Pipe {
    fn from_char(c: char) -> Pipe {
        match c {
            '|' => Pipe::VERTICAL,
            '-' => Pipe::HORIZONTAL,
            'L' => Pipe::L,
            'J' => Pipe::J,
            '7' => Pipe::SEVEN,
            'F' => Pipe::F,
            '.' => Pipe::GROUND,
            'S' => Pipe::START,
            _ => panic!("Invalid pipe char"),
        }
    }

    fn is_valid(&self, direction: &Direction) -> bool {
        match self {
            Pipe::VERTICAL => match direction {
                Direction::UP | Direction::DOWN => true,
                _ => false,
            },
            Pipe::HORIZONTAL => match direction {
                Direction::LEFT | Direction::RIGHT => true,
                _ => false,
            },
            Pipe::L => match direction {
                Direction::UP | Direction::RIGHT => true,
                _ => false,
            },
            Pipe::J => match direction {
                Direction::UP | Direction::LEFT => true,
                _ => false,
            },
            Pipe::SEVEN => match direction {
                Direction::DOWN | Direction::LEFT => true,
                _ => false,
            },
            Pipe::F => match direction {
                Direction::DOWN | Direction::RIGHT => true,
                _ => false,
            },
            Pipe::GROUND => false,
            Pipe::START => true,
        }
    }
}

fn main() {
    let raw = std::fs::read_to_string("input.txt").expect("could not read file");

    let content: Vec<&str> = raw.lines().filter(|line| !line.is_empty()).collect();

    let game = get_game(content);

    let path = solve_p1(&game);
    println!("Part 1: {}", path.len() as u32 / 2);
    println!("Part 2: {}", solve_p2(path));
}

fn get_game(content: Vec<&str>) -> Game {
    let mut game = Game {
        maze: Vec::new(),
        starting_point: Point {
            x: 0,
            y: 0,
            pipe: Pipe::GROUND,
        },
    };

    for (i, line) in content.iter().enumerate() {
        let mut row: Vec<Point> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            let point = Point {
                x: j as i32,
                y: i as i32,
                pipe: Pipe::from_char(c),
            };
            row.push(point);
            if c == 'S' {
                game.starting_point = point.clone();
            }
        }
        game.maze.push(row);
    }

    game
}

fn solve_p1(game: &Game) -> Vec<Point> {
    let mut visited: Vec<Point> = Vec::new();
    let mut stack: Vec<Point> = Vec::new();

    stack.push(game.starting_point);

    let directions = vec![
        Direction::UP,
        Direction::DOWN,
        Direction::LEFT,
        Direction::RIGHT,
    ];

    while !stack.is_empty() {
        let current = stack.pop().unwrap();

        visited.push(current.clone());

        for direction in directions.iter() {
            if current.pipe.is_valid(direction) {
                let next = match direction {
                    Direction::UP => {
                        if current.y > 0 {
                            Point {
                                x: current.x,
                                y: current.y - 1,
                                pipe: game.maze[current.y as usize - 1][current.x as usize].pipe,
                            }
                        } else {
                            continue;
                        }
                    }
                    Direction::DOWN => {
                        if current.y < game.maze.len() as i32 - 1 {
                            Point {
                                x: current.x,
                                y: current.y + 1,
                                pipe: game.maze[current.y as usize + 1][current.x as usize].pipe,
                            }
                        } else {
                            continue;
                        }
                    }
                    Direction::LEFT => {
                        if current.x > 0 {
                            Point {
                                x: current.x - 1,
                                y: current.y,
                                pipe: game.maze[current.y as usize][current.x as usize - 1].pipe,
                            }
                        } else {
                            continue;
                        }
                    }
                    Direction::RIGHT => {
                        if current.x < game.maze[0].len() as i32 - 1 {
                            Point {
                                x: current.x + 1,
                                y: current.y,
                                pipe: game.maze[current.y as usize][current.x as usize + 1].pipe,
                            }
                        } else {
                            continue;
                        }
                    }
                };

                if next.pipe == Pipe::GROUND {
                    continue;
                }

                if next.pipe == Pipe::START && visited.len() > 100 {
                    return visited;
                }

                if !is_visited(&visited, &next) {
                    stack.push(next);
                }
            }
        }
    }

    visited
}

fn solve_p2(path: Vec<Point>) -> f64 {
    let n = path.len();
    let mut sum = 0.0;

    for i in 0..n {
        let current = &path[i];
        let next = &path[(i + 1) % n];

        sum += (current.x as f64 * next.y as f64) - (next.x as f64 * current.y as f64);
    }

    let area = sum.abs() / 2.0;

    let dots = area - (n as f64 / 2.0) + 1.0;

    dots.round()
}

fn is_visited(visited: &Vec<Point>, point: &Point) -> bool {
    for p in visited.iter() {
        if p.x == point.x && p.y == point.y {
            return true;
        }
    }

    false
}
