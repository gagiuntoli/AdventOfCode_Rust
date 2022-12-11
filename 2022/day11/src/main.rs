use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Prod(u64),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    results: (u64, u64),
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let file: String = fs::read_to_string(filename)?.parse()?;

    let mut monkeys_initial = Vec::<Monkey>::new();

    file.trim().split("\n\n").for_each(|section| {
        let mut items: Option<Vec<u64>> = None;
        let mut operation: Option<Operation> = None;
        let mut divisor: Option<u64> = None;
        let mut true_val: Option<u64> = None;
        let mut false_val: Option<u64> = None;

        section.split('\n').enumerate().for_each(|(i, line)| {
            let splitted_line = line.trim().split(" ").collect::<Vec<&str>>();
            match splitted_line[0..2] {
                ["Monkey", _] => (),
                ["Starting", _] => {
                    items = Some(
                        splitted_line[2..]
                            .into_iter()
                            .map(|x| x.replace(",", "").parse::<u64>().unwrap())
                            .collect::<Vec<u64>>(),
                    );
                }
                ["Operation:", _] => {
                    match splitted_line[4..] {
                        ["*", "old"] => operation = Some(Operation::Square),
                        ["*", val] => {
                            operation = Some(Operation::Prod(val.parse::<u64>().unwrap()))
                        }
                        ["+", val] => operation = Some(Operation::Add(val.parse::<u64>().unwrap())),
                        _ => panic!("invalid operation"),
                    };
                }
                ["Test:", _] => divisor = Some(splitted_line[3].parse::<u64>().unwrap()),
                ["If", "true:"] => true_val = Some(splitted_line[5].parse::<u64>().unwrap()),
                ["If", "false:"] => false_val = Some(splitted_line[5].parse::<u64>().unwrap()),
                _ => println!("invalid parsing: {}", i),
            };
        });

        let monkey = Monkey {
            items: items.unwrap(),
            operation: operation.unwrap(),
            divisor: divisor.unwrap(),
            results: (true_val.unwrap(), false_val.unwrap()),
        };
        monkeys_initial.push(monkey);
    });

    let mut monkeys = monkeys_initial.clone();
    let mut inspected_items = vec![0u64; monkeys.len()];

    (0..20).for_each(|_| {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            monkey.items.iter().for_each(|item| {
                let num = match monkeys[i].operation {
                    Operation::Add(val) => item + val,
                    Operation::Prod(val) => item * val,
                    Operation::Square => item * item,
                };
                let new_item = num / 3;
                if new_item % monkey.divisor == 0 {
                    monkeys[monkey.results.0 as usize].items.push(new_item);
                } else {
                    monkeys[monkey.results.1 as usize].items.push(new_item);
                }
            });
            inspected_items[i] += monkeys[i].items.len() as u64;
            monkeys[i].items = vec![];
        }
    });

    inspected_items.sort();
    println!(
        "{}",
        inspected_items[inspected_items.len() - 2..]
            .iter()
            .fold(1, |acc, elem| acc * elem)
    );

    let mut monkeys = monkeys_initial.clone();
    let mut inspected_items = vec![0u64; monkeys.len()];
    let prime_divisors = monkeys.iter().fold(1, |acc, monkey| acc * monkey.divisor);

    (0..10000).for_each(|_| {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            monkey.items.iter().for_each(|item| {
                let num = match monkeys[i].operation {
                    Operation::Add(val) => item + val,
                    Operation::Prod(val) => item * val,
                    Operation::Square => item * item,
                };
                if num % monkey.divisor == 0 {
                    monkeys[monkey.results.0 as usize]
                        .items
                        .push(num % prime_divisors);
                } else {
                    monkeys[monkey.results.1 as usize]
                        .items
                        .push(num % prime_divisors);
                }
            });
            inspected_items[i] += monkeys[i].items.len() as u64;
            monkeys[i].items = vec![];
        }
    });

    inspected_items.sort();

    println!(
        "{}",
        inspected_items[inspected_items.len() - 2..]
            .iter()
            .fold(1, |acc, elem| acc * elem)
    );

    Ok(())
}
