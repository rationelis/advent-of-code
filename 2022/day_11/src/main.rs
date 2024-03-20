use utils::read_input_file;

#[derive(Debug, Clone)]
enum Operation {
    Multiply,
    Add,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i32>,
    operation: (Operation, i32),
    test_divisible_by: i32,
    throw_to_if_true: usize,
    throw_to_if_false: usize,
    items_inserted: i32,
}

impl Monkey {
    fn add_item(&mut self, item: i32) {
        self.items.push(item);
    }

    fn remove_item(&mut self, item: i32) {
        self.items.retain(|x| x != &item);
    }
}

fn main() {
    let lines = read_input_file("input.txt").unwrap();

    let monkeys = get_monkeys(lines);

    let part1 = handle_rounds(&mut monkeys.clone(), 20, false);
    let part2 = handle_rounds(&mut monkeys.clone(), 10000, true);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn handle_rounds(monkeys: &mut Vec<Monkey>, rounds: i32, should_modulo: bool) -> i64 {
    let mut reducer = 3;
    if should_modulo {
        reducer = monkeys.iter().map(|x| x.test_divisible_by).product();
    }
    for _ in 0..rounds {
        for index in 0..monkeys.len() {
            let updates = handle_round(monkeys.get(index).unwrap(), should_modulo, reducer);
            for ((new, old), throw_to) in updates {
                monkeys[index].remove_item(old);
                monkeys[index].items_inserted += 1;
                monkeys[throw_to].add_item(new);
            }
        }
    }
    let mut items_inserted = monkeys.iter().map(|x| x.items_inserted).collect::<Vec<i32>>();
    items_inserted.sort();
    items_inserted.pop().unwrap() as i64 * items_inserted.pop().unwrap() as i64
}

fn handle_round(monkey: &Monkey, should_modulo: bool, reducer: i32) -> Vec<((i32, i32), usize)> {
    let mut updates = Vec::new();
    for item in monkey.items.iter() {
        let mut worry_level = *item as i64;
        let mut operation_value = monkey.operation.1 as i64;
        if operation_value == 0 {
            operation_value = worry_level;
        }
        match monkey.operation.0 {
            Operation::Multiply => worry_level *= operation_value,
            Operation::Add => worry_level += operation_value,
        }
        if should_modulo {
            worry_level %= reducer as i64;
        } else {
            worry_level /= reducer as i64;
        }
        if worry_level % monkey.test_divisible_by as i64 == 0 {
            updates.push(((worry_level as i32, *item), monkey.throw_to_if_true));
        } else {
            updates.push(((worry_level as i32, *item), monkey.throw_to_if_false));
        }       
    }
    updates
}

fn get_monkeys(lines: Vec<String>) -> Vec<Monkey> {
    lines.split(|line| line.is_empty())
        .enumerate()
        .map(|(_id, block)| {
            let mut monkey = Monkey {
                items: Vec::new(),
                operation: (Operation::Multiply, 0),
                test_divisible_by: 0,
                throw_to_if_true: 0,
                throw_to_if_false: 0,
                items_inserted: 0,
            };

            for (i, line) in block.iter().enumerate().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();

                match i {
                    1 => {
                        let items: Vec<i32> = parts[2..]
                            .iter()
                            .map(|x| x.replace(",", "").parse().unwrap())
                            .collect();
                        monkey.items.extend(items);
                    }
                    2 => {
                        let operation = match parts[4] {
                            "*" => Operation::Multiply,
                            "+" => Operation::Add,
                            _ => panic!("Invalid operation"),
                        };
                        let value = match parts[5] {
                            "old" => 0,
                            _ => parts[5].parse().unwrap(),
                        };
                        monkey.operation = (operation, value);
                    }
                    3 => monkey.test_divisible_by = parts[3].parse().unwrap(),
                    4 => monkey.throw_to_if_true = parts[5].parse().unwrap(),
                    5 => monkey.throw_to_if_false = parts[5].parse().unwrap(),
                    _ => {}
                }
            }
            monkey
        })
        .collect()
}

