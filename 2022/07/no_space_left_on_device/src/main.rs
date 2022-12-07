use std::{collections::HashMap, fs};

fn main() {
    let filename = "input.txt";
    let file = fs::read_to_string(filename).expect("Could not read file {file_path}");
    let lines = file.trim().split('\n').collect::<Vec<&str>>();

    let mut dir_size = HashMap::<Vec<&str>, i32>::new();
    let mut path: Vec<&str> = Vec::new();

    for line in lines {
        let line = line.split(' ').collect::<Vec<&str>>();

        if line == ["$", "cd", ".."] {
            path.pop();
        } else if line.len() == 3 {
            path.push(line[2]);
        } else if line.len() == 2 && line[0] != "$" && line[0] != "dir" {
            // Increase size of directory
            let size = line[0].parse::<i32>().unwrap();
            *dir_size.entry(path.to_owned()).or_default() += size;

            //Increase size of all parent directories
            let mut parent_path = path.split_last().unwrap().1.to_vec();

            while !parent_path.is_empty() {
                *dir_size.entry(parent_path.to_owned()).or_default() += size;
                parent_path.pop();
            }
        }
    }

    let sizes = dir_size.iter().map(|(_, v)| *v).collect::<Vec<i32>>();

    println!(
        "Task 1 sum: {}",
        sizes.iter().filter(|size| **size <= 100_000).sum::<i32>()
    );

    let free_space = 70_000_000 - dir_size.get(&vec!["/"]).unwrap();
    println!("Free space available: {free_space}");

    println!(
        "Freeing size of: {}",
        sizes
            .iter()
            .filter(|size| **size + free_space >= 30_000_000)
            .min()
            .unwrap()
    );
}
