use std::fs;

fn main() {
    let filename = "input.txt";
    let file = fs::read_to_string(filename).expect("Could not read file {file_path}");
    let words = file.trim().split("\n").collect::<Vec<&str>>();

    let mut sum = 0;
    let mut sums = Vec::new();
    for word in words {
        if word == "" {
            sums.push(sum);
            sum = 0;
            continue;
        }

        sum += word.parse::<i32>().unwrap();
    }

    sums.sort();
    sums.reverse();
    println!("Biggest 1: {}", sums.iter().max().unwrap());
    println!("Biggest 3: {:?}", &sums[0..3].iter().sum::<i32>());
}
