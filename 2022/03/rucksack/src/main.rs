use std::fs;

fn main() {
    let filename = "input.txt";
    let file = fs::read_to_string(filename).expect("Could not read file {file_path}");
    let lines = file.trim().split("\n").collect::<Vec<&str>>();

    task1(&lines);
    task2(&lines);
}

fn item_priority(item: char) -> i32 {
    if item.is_ascii_lowercase() {
        return item as i32 - 96;
    } else {
        return item as i32 - 38;
    }
}

fn task1(lines: &Vec<&str>) {
    let mut sum = 0;
    for line in lines {
        let mid = line.len() / 2;
        let (left, right) = line.split_at(mid);

        for c in left.chars() {
            if right.contains(c) {
                sum += item_priority(c);
                break;
            }
        }
    }

    println!("Task 1 sum: {sum}");
}

fn task2(lines: &Vec<&str>) {
    let mut i = 0;
    let mut sum = 0;

    while i < lines.len() {
        let chunk = &lines[i..i + 3];

        for c in chunk[0].chars() {
            if chunk[1].contains(c) && chunk[2].contains(c) {
                sum += item_priority(c);
                break;
            }
        }

        i += 3;
    }

    println!("Task 2 sum: {sum}");
}
