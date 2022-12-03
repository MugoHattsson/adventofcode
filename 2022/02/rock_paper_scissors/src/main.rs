use std::fs;

const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSS: i32 = 0;

fn main() {
    let filename = "input.txt";
    let file = fs::read_to_string(filename).expect("Could not read file {file_path}");
    let lines = file.trim().split("\n").collect::<Vec<&str>>();

    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in lines {
        let round = line.trim().split(' ').collect::<Vec<&str>>();

        let result = strategy1(&round);
        let choice = match round[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => -1,
        };

        sum1 += result + choice;
        sum2 += strategy2(&round);
    }

    println!("Sum: {sum1}, {sum2}");
}

// Rock A, X: 1p
// Paper B, Y: 2p
// Scissors C, Z: 3p
fn strategy1(round: &Vec<&str>) -> i32 {
    let opponent = round[0];
    let player = round[1];

    match opponent {
        "A" => match player {
            "Y" => WIN,
            "Z" => LOSS,
            "X" => DRAW,
            _ => -1,
        },
        "B" => match player {
            "Z" => WIN,
            "X" => LOSS,
            "Y" => DRAW,
            _ => -1,
        },
        "C" => match player {
            "X" => WIN,
            "Y" => LOSS,
            "Z" => DRAW,
            _ => -1,
        },
        _ => -1,
    }
}

// X: LOSE
// Y: DRAW
// Z: WIN
fn strategy2(round: &Vec<&str>) -> i32 {
    let opponent = round[0];
    let goal = round[1];

    match goal {
        // LOSE
        "X" => match opponent {
            "A" => 3,
            "B" => 1,
            "C" => 2,
            _ => -1,
        },
        // DRAW
        "Y" => match opponent {
            "A" => 1 + DRAW,
            "B" => 2 + DRAW,
            "C" => 3 + DRAW,
            _ => -1,
        },
        // WIN
        "Z" => match opponent {
            "A" => 2 + WIN,
            "B" => 3 + WIN,
            "C" => 1 + WIN,
            _ => -1,
        },
        _ => -1,
    }
}
