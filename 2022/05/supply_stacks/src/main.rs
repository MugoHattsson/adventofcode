use std::fs;

fn main() {
    let filename = "input.txt";
    let file = fs::read_to_string(filename).expect("Could not read file {file_path}");
    let lines = file.split("\n").collect::<Vec<&str>>();

    task1(&lines);
    task2(&lines);
}

fn task1(lines: &Vec<&str>) {
    let initial_height = lines.iter().position(|x| x.contains("1")).unwrap();
    let num_stacks = *lines
        .iter()
        .find(|x| x.contains("1"))
        .unwrap()
        .trim()
        .split(" ")
        .filter(|&x| x != "")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .last()
        .unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    init_stacks(&mut stacks, lines, initial_height, num_stacks);

    for line in lines[(initial_height + 1)..].iter() {
        if *line == "" {
            continue;
        }
        let line = line
            .split(" ")
            .flat_map(|x| x.parse::<usize>())
            .collect::<Vec<usize>>();

        let from = line[1] - 1;
        let to = line[2] - 1;

        for _ in 0..line[0] {
            let x = stacks[from].pop().unwrap();
            stacks[to].push(x);
        }
    }

    print!("Cratemover 9000: ");
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!("");
}

fn task2(lines: &Vec<&str>) {
    let initial_height = lines.iter().position(|x| x.contains("1")).unwrap();
    let num_stacks: usize = *lines
        .iter()
        .find(|x| x.contains("1"))
        .unwrap()
        .trim()
        .split(" ")
        .filter(|&x| x != "")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .last()
        .unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    init_stacks(&mut stacks, lines, initial_height, num_stacks);

    for line in lines[(initial_height + 1)..].iter() {
        if *line == "" {
            continue;
        }
        let line = line
            .split(" ")
            .flat_map(|x| x.parse::<usize>())
            .collect::<Vec<usize>>();

        let from = line[1] - 1;
        let to = line[2] - 1;

        let mut temp_stack: Vec<char> = Vec::new();
        for _ in 0..line[0] {
            let x = stacks[from].pop().unwrap();
            temp_stack.push(x);
        }
        for _ in 0..line[0] {
            let x = temp_stack.pop().unwrap();
            stacks[to].push(x);
        }
    }

    print!("Cratemover 9001: ");
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!("");
}

fn init_stacks(
    stacks: &mut Vec<Vec<char>>,
    lines: &Vec<&str>,
    stack_height: usize,
    num_stacks: usize,
) {
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
