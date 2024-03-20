use utils::read_input_file;

/*
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
*/

#[derive(Debug, Clone)]
enum Operation {
    Multiply,
    Add,
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<i32>,
    operation: (Operation, i32),
    test_divisible_by: i32,
    throw_to_if_true: usize,
    throw_to_if_false: usize,
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

    let mut monkeys = get_monkeys(lines);

    print_monkey_items(&monkeys);

    let rounds = 1;
    for _ in 0..rounds {
        for monkey in monkeys.iter() {
            let updates = handle_round(monkey);
            for ((worry_level, item), throw_to) in updates {
                // Borrow mut to remove item
                monkeys[monkey.id].remove_item(item);

                //monkeys[throw_to].add_item(worry_level);
            }
        }
    }

    print_monkey_items(&monkeys);
}

fn print_monkey_items(monkeys: &Vec<Monkey>) {
    for monkey in monkeys.iter() {
        println!("Monkey {}: {:?}", monkey.id, monkey.items);
    }
}

fn handle_round(monkey: &Monkey) -> Vec<((i32, i32), usize)> {
    let mut updates = Vec::new();
    for item in monkey.items.iter() {
        let mut worry_level = *item;
        let mut operation_value = monkey.operation.1;
        if operation_value == 0 {
            operation_value = worry_level;
        }
        match monkey.operation.0 {
            Operation::Multiply => worry_level *= operation_value,
            Operation::Add => worry_level += operation_value,
        }
        worry_level /= 3;
        if worry_level % monkey.test_divisible_by == 0 {
            updates.push(((worry_level, *item), monkey.throw_to_if_true));
        } else {
            updates.push(((worry_level, *item), monkey.throw_to_if_false));
        }       
    }
    updates
}

fn get_monkeys(lines: Vec<String>) -> Vec<Monkey> {
    lines.split(|line| line.is_empty())
        .enumerate()
        .map(|(id, block)| {
            let mut monkey = Monkey {
                id,
                items: Vec::new(),
                operation: (Operation::Multiply, 0),
                test_divisible_by: 0,
                throw_to_if_true: 0,
                throw_to_if_false: 0,
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

