use utils::read_input_file;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Structure {
    rocks: HashSet<Point>,
}

impl Structure {
    fn new() -> Structure {
        Structure {
            rocks: HashSet::new(),
        }
    }

    fn add_rock(&mut self, rock: Point) {
        self.rocks.insert(rock);
    }

    fn extend_range(&mut self, x: i32, y: i32, diff: i32, is_horizontal: bool) {
        let step = if diff > 0 { -1 } else { 1 };
        let range = if diff > 0 { diff - 1 } else { diff + 1 };

        for i in 1..range.abs()+1 {
            if is_horizontal {
                self.rocks.insert(Point { x: x + i*step, y });
            } else {
                self.rocks.insert(Point { x, y: y + i*step });
            }
        }
    }

    fn is_blocked(&self, point: Point) -> bool {
        self.rocks.contains(&point)
    }
}

fn main() {
    let lines = read_input_file("input.txt").unwrap();

    let mut structure = generate_structure(lines);

    let starting_point = Point { x: 500, y: 0 };

    let mut sand: HashSet<Point> = HashSet::new();

    let mut reached_bottom = false;

    loop {
        if reached_bottom {
            break;
        }

        let mut grain = starting_point;

        loop {
            let down = Point { x: grain.x, y: grain.y + 1 };

            if down.y > 2000 {
                reached_bottom = true;
                break;
            }

            if !structure.is_blocked(down) {
                grain = down;
                continue;
            }

            let down_left = Point { x: grain.x - 1, y: grain.y + 1 };

            if !structure.is_blocked(down_left) {
                grain = down_left;
                continue;
            }

            let down_right = Point { x: grain.x + 1, y: grain.y + 1 };

            if !structure.is_blocked(down_right) {
                grain = down_right;
                continue;
            }

            sand.insert(grain);
            structure.add_rock(grain);
            break;
        }
    }

    println!("Sand grains: {}", sand.len());
}

fn generate_structure(lines: Vec<String>) -> Structure {
    let mut structure = Structure::new();

    let all_structures = lines.iter().map(|line| line.split(" -> ").collect::<Vec<&str>>());
    
    for s in all_structures {
        let ranges: Vec<Point> = s.iter().map(|part| {
            let parts = part.split(",").collect::<Vec<&str>>();
            let x = parts[0].parse::<i32>().unwrap();
            let y = parts[1].parse::<i32>().unwrap();
            Point { x, y }
        }).collect();
        
        ranges.iter().for_each(|point| {
            structure.add_rock(point.clone());
        });

        for i in 0..ranges.len()-1 {
            let (x1, y1) = (ranges[i].x, ranges[i].y);
            let (x2, y2) = (ranges[i+1].x, ranges[i+1].y);

            let diff_x = x1 - x2;
            let diff_y = y1 - y2;

            if diff_x != 0 {
                structure.extend_range(x1, y1, diff_x, true);
            }

            if diff_y != 0 {
                structure.extend_range(x1, y1, diff_y, false);
            }
        }
    } 

    structure 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_cave() {
        let input_vec = vec!["498,4 -> 498,6 -> 496,6"];
        let structure = generate_structure(input_vec.iter().map(|s| s.to_string()).collect());
        
        assert_eq!(structure.rocks.len(), 5);
        assert!(structure.rocks.contains(&Point { x: 498, y: 4 }));
        assert!(structure.rocks.contains(&Point { x: 498, y: 5 }));
        assert!(structure.rocks.contains(&Point { x: 498, y: 6 }));
        assert!(structure.rocks.contains(&Point { x: 497, y: 6 }));
        assert!(structure.rocks.contains(&Point { x: 496, y: 6 }));
    }

    #[test]
    fn test_generate_cave2() {
        let input_vec = vec!["503,4 -> 502,4 -> 502,9 -> 494,9"];
        let structure = generate_structure(input_vec.iter().map(|s| s.to_string()).collect());
        
        assert_eq!(structure.rocks.len(), 15);
        assert!(structure.rocks.contains(&Point { x: 503, y: 4 }));
        assert!(structure.rocks.contains(&Point { x: 502, y: 4 }));
        assert!(structure.rocks.contains(&Point { x: 502, y: 5 }));
        assert!(structure.rocks.contains(&Point { x: 502, y: 6 }));
        assert!(structure.rocks.contains(&Point { x: 502, y: 7 }));
        assert!(structure.rocks.contains(&Point { x: 502, y: 8 }));
        assert!(structure.rocks.contains(&Point { x: 502, y: 9 }));
        assert!(structure.rocks.contains(&Point { x: 501, y: 9 }));
        assert!(structure.rocks.contains(&Point { x: 500, y: 9 }));
        assert!(structure.rocks.contains(&Point { x: 499, y: 9 }));
        assert!(structure.rocks.contains(&Point { x: 498, y: 9 }));
        assert!(structure.rocks.contains(&Point { x: 497, y: 9 }));
        assert!(structure.rocks.contains(&Point { x: 496, y: 9 }));
        assert!(structure.rocks.contains(&Point { x: 495, y: 9 }));
        assert!(structure.rocks.contains(&Point { x: 494, y: 9 }));
    }
}

