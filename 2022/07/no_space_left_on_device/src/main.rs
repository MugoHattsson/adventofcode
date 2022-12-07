use std::{collections::HashMap, fs};

fn main() {
    let filename = "input.txt";
    let file = fs::read_to_string(filename).expect("Could not read file {file_path}");
    let lines = file.trim().split("\n").collect::<Vec<&str>>();

    let mut dir_size = HashMap::<Vec<&str>, i32>::new();
    let mut root: Vec<&str> = Vec::new();

    for line in lines {
        let line: Vec<_> = line.split(' ').collect();
        let _ = match line[..] {
            ["$", "cd", ".."] => root.pop().unwrap(),
            ["$", "cd", p] => {
                root.push(p);
                continue;
            }
            ["$", "ls"] => continue,
            ["dir", _] => continue,
            [size, _] => {
                let size = size.parse::<i32>().unwrap();
                if dir_size.contains_key(&root) {
                    let mut temp = *dir_size.get(&root).unwrap();
                    temp += size;
                    dir_size.insert(root.clone(), temp);
                } else {
                    dir_size.insert(root.clone(), size);
                }

                let mut path = root.clone();
                path.pop();

                while !path.is_empty() {
                    if dir_size.contains_key(&path) {
                        let mut temp = *dir_size.get(&path).unwrap();
                        temp += size;
                        dir_size.insert(path.clone(), temp);
                    } else {
                        dir_size.insert(path.clone(), size);
                    }
                    path.pop();
                }

                continue;
            }
            _ => continue,
        };
    }

    let sizes = dir_size.iter().map(|(_, v)| *v).collect::<Vec<i32>>();

    println!(
        "Task 1 sum: {}",
        sizes.iter().filter(|v| **v <= 100000).sum::<i32>()
    );

    let free = 70_000_000 - dir_size.get(&vec!["/"]).unwrap();
    println!("Free space available: {free}");

    println!(
        "Freeing size of: {}",
        sizes
            .iter()
            .filter(|v| **v + free >= 30_000_000)
            .min()
            .unwrap()
    );
}
