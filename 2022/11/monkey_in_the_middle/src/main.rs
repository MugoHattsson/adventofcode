use std::{collections::VecDeque, fs};

const WORRIED: bool = true; // True for task 2

fn main() {
    let filename = "input.txt";
    let file = fs::read_to_string(filename).expect("Could not read file {file_path}");
    let monkeys = file.trim().split("\n\n").collect::<Vec<&str>>();

    let mut worry_levels: Vec<VecDeque<u64>> = Vec::new();
    let mut ops: Vec<Vec<&str>> = Vec::new();
    let mut tests: Vec<Vec<u64>> = Vec::new();
    let mut inspections: Vec<u64> = Vec::new();

    // Parse monkeys
    for monkey in monkeys {
        let monkey = monkey
            .trim()
            .split('\n')
            .skip(1)
            .map(|line| line.trim())
            .collect::<Vec<&str>>();

        // Starting items
        worry_levels.push(
            monkey[0]["Starting items: ".len()..]
                .split(", ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<VecDeque<_>>(),
        );

        // Operation
        ops.push(
            monkey[1]["Operation: new = ".len()..]
                .split(' ')
                .skip(1)
                .collect::<Vec<&str>>(),
        );

        // Test
        tests.push(
            monkey[2..]
                .iter()
                .map(|line| line.split(' ').last().unwrap().parse::<u64>().unwrap())
                .collect::<Vec<_>>(),
        );

        inspections.push(0);
    }

    // Calculate monkey business
    let num_monkeys = worry_levels.len();
    let chinese_remainder: u64 = tests.iter().map(|x| x[0]).product();

    let num_rounds = if WORRIED { 10_000 } else { 20 };

    for _round in 0..num_rounds {
        for m in 0..num_monkeys {
            let op = ops[m].clone();
            let test = tests[m].clone();

            while !worry_levels[m].is_empty() {
                // Inspection and boredom
                inspections[m] += 1;
                let mut item = worry_levels[m].pop_front().unwrap();

                if op[0] == "+" {
                    if let Ok(num) = op[1].parse::<u64>() {
                        item += num;
                        item %= chinese_remainder;
                    } else {
                        item += item;
                        item %= chinese_remainder;
                    }
                } else if op[0] == "*" {
                    if let Ok(num) = op[1].parse::<u64>() {
                        item *= num;
                        item %= chinese_remainder;
                    } else {
                        item *= item;
                        item %= chinese_remainder;
                    }
                }

                if !WORRIED {
                    item /= 3;
                }

                // Test
                if item % test[0] == 0 {
                    worry_levels[test[1] as usize].push_back(item);
                } else {
                    worry_levels[test[2] as usize].push_back(item);
                }
            }
        }
    }

    inspections.sort();
    println!(
        "Total monkey business = {}",
        inspections.iter().rev().take(2).product::<u64>()
    );
}
