use std::fs;
use std::ops::RangeInclusive;

fn main() {
    let filename = "input.txt";
    let file = fs::read_to_string(filename).expect("Could not read file {file_path}");
    let lines = file.trim().split('\n').collect::<Vec<&str>>();

    let mut num_contained = 0;
    let mut num_overlap = 0;

    for line in lines {
        let line = line
            .split(',')
            .flat_map(|x| x.split('-'))
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let first = line[0]..=line[1];
        let second = line[2]..=line[3];

        // Contained
        if size(&first) > size(&second) {
            if first.start() <= second.start() && first.end() >= second.end() {
                num_contained += 1;
            }
        } else if first.start() >= second.start() && first.end() <= second.end() {
            num_contained += 1;
        }

        // Overlaps
        for i in first {
            if second.contains(&i) {
                num_overlap += 1;
                break;
            }
        }
    }

    println!("Contained: {num_contained}");
    println!("Overlaps: {num_overlap}");
}

fn size(range: &RangeInclusive<i32>) -> i32 {
    range.end() - range.start()
}
