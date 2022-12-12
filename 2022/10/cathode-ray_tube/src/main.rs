use std::fs;

fn main() {
    let filename = "input.txt";
    let file = fs::read_to_string(filename).expect("Could not read file {file_path}");
    let instructions = file.trim().split('\n').collect::<Vec<&str>>();

    let mut x = 1;
    let mut signal: Vec<i32> = Vec::new();

    while signal.len() < 6 * 40 {
        for instr in instructions.iter() {
            let instr = instr.split(' ').collect::<Vec<&str>>();

            if instr[0] == "noop" {
                signal.push(x);
            } else {
                signal.push(x);
                signal.push(x);
                x += instr[1].parse::<i32>().unwrap();
            }
        }
    }

    println!(
        "Signal strength sum: {}\n",
        (20..=220)
            .step_by(40)
            .map(|i| i as i32 * signal[i - 1])
            .sum::<i32>()
    );

    println!("CRT output");
    for row in 0..6 {
        for col in 0..40 {
            let i = row * 40 + col;
            let x = signal[i];

            print!(
                "{}",
                if (x - 1..=x + 1).contains(&(col as i32)) {
                    "#"
                } else {
                    "."
                }
            );
        }
        println!();
    }
}
