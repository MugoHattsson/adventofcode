use std::fs;

fn main() {
    let filename = "input.txt";
    let file = fs::read_to_string(filename).expect("Could not read file {file_path}");
    let lines: Vec<_> = file.split('\n').collect();

    let stack_height = lines.iter().position(|x| x.contains('1')).unwrap();
    let num_stacks: usize = *lines
        .iter()
        .find(|x| x.contains('1'))
        .unwrap()
        .trim()
        .split(' ')
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse::<_>().unwrap())
        .collect::<Vec<_>>()
        .last()
        .unwrap();

    let mut stacks_9000: Vec<Vec<char>> = Vec::new();
    let mut stacks_9001: Vec<Vec<char>> = Vec::new();
    create_initial_stacks(&mut stacks_9000, &lines, stack_height, num_stacks);
    create_initial_stacks(&mut stacks_9001, &lines, stack_height, num_stacks);

    let procedure = lines[(stack_height + 1..)]
        .iter()
        .filter(|step| !step.is_empty());

    // Moving strategy for cratemover 9000
    // Move one crate at a time, changing the stack order
    for step in procedure.clone() {
        let line = step
            .split(' ')
            .flat_map(|x| x.parse::<usize>())
            .collect::<Vec<usize>>();

        let from = line[1] - 1;
        let to = line[2] - 1;

        for _ in 0..line[0] {
            let x = stacks_9000[from].pop().unwrap();
            stacks_9000[to].push(x);
        }
    }

    // Moving strategy for cratemover 9001
    // Move several crates at once, keeping their order
    for step in procedure {
        let line: Vec<usize> = step
            .split(' ')
            .flat_map(|x| x.parse::<usize>())
            .collect::<Vec<usize>>();

        let from = line[1] - 1;
        let to = line[2] - 1;

        let idx = stacks_9001[from].len() - line[0];
        let crates: Vec<_> = stacks_9001[from].drain(idx..).collect();
        for c in crates {
            stacks_9001[to].push(c);
        }
    }

    print_top_crates("9000", stacks_9000);
    print_top_crates("9001", stacks_9001);
}

fn create_initial_stacks(
    stacks: &mut Vec<Vec<char>>,
    lines: &[&str],
    stack_height: usize,
    num_stacks: usize,
) {
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }
    let mut col: usize = 0;
    while col < num_stacks {
        let mut row = stack_height as i32;

        while row > -1 {
            let c = lines[row as usize].as_bytes()[col * 4 + 1] as char;
            if c.is_alphabetic() {
                stacks[col].push(c);
            }
            row -= 1;
        }
        col += 1;
    }
}

fn print_top_crates(version: &str, stacks: Vec<Vec<char>>) {
    print!("Cratemover {version}: ");
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}
