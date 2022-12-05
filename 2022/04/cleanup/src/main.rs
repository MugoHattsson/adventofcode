use std::fs;

fn main() {
    let filename = "input.txt";
    let file = fs::read_to_string(filename).expect("Could not read file {file_path}");
    let lines = file.trim().split("\n").collect::<Vec<&str>>();

    let mut num_contained = 0;
    let mut num_overlap = 0;

    for line in lines {
        let something = line
            .split(",")
            .flat_map(|x| x.split("-"))
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let first = (something[0], something[1]);
        let second = (something[2], something[3]);

        // Contained
        if size(first) > size(second) {
            if first.0 <= second.0 && first.1 >= second.1 {
                num_contained += 1;
            }
        } else {
            if first.0 >= second.0 && first.1 <= second.1 {
                num_contained += 1;
            }
        }

        let range1 = first.0..=first.1;
        let range2 = second.0..=second.1;

        // Overlaps
        for i in range1 {
            if range2.contains(&i) {
                num_overlap += 1;
                break;
            }
        }
    }

    println!("Contained: {num_contained}");
    println!("Overlaps: {num_overlap}");
}

fn size(tuple: (i32, i32)) -> i32 {
    i32::abs(tuple.0 - tuple.1)
}
