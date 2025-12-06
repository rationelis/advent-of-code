use utils::read_input_file;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Structure {
    rocks: HashSet<Point>,
    highest_y: i32,
}

impl Structure {
    fn new() -> Structure {
        Structure {
            rocks: HashSet::new(),
            highest_y: 0,
        }
    }

    fn add_rock(&mut self, rock: Point) {
        if rock.y > self.highest_y {
            self.highest_y = rock.y;
        } 
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

    let structure = generate_structure(lines);

    let part1 = pour_sand(&mut structure.clone(), false); 
    let part2 = pour_sand(&mut structure.clone(), true);
    
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn pour_sand(structure: &mut Structure, has_floor: bool) -> i32 {
    if has_floor {
        let floor_y = structure.highest_y + 2;
        for x in -2000..2000 {
            structure.add_rock(Point { x, y: floor_y });
        }
    }

    let starting_point = Point { x: 500, y: 0 };

    let mut sand: HashSet<Point> = HashSet::new();

    let mut reached_bottom = false;

    loop {
        if reached_bottom {
            break;
        }

        let mut grain = starting_point;

        loop {
            if structure.is_blocked(grain) {
                reached_bottom = true;
                break;
            }

            let down = Point { x: grain.x, y: grain.y + 1 };

            if down.y > 2000 && !has_floor {
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

    sand.len() as i32
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

