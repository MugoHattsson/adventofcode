use std::fs;

fn main() {
    let filename = "input.txt";
    let file = fs::read_to_string(filename).expect("Could not read file {file_path}");
    let datastream = file.trim().as_bytes();

    println!(
        "Packet start found at: {}",
        find_sequence(datastream, 4).unwrap()
    );

    println!(
        "Message start found at: {}",
        find_sequence(datastream, 14).unwrap()
    );
}

fn find_sequence(datastream: &[u8], size: usize) -> Option<usize> {
    for i in 0..datastream.len() - size {
        if all_different(&datastream[i..i + size]) {
            return Some(i + size);
        }
    }
    None
}

fn all_different(window: &[u8]) -> bool {
    for (i, c) in window.iter().enumerate() {
        for other in window.iter().skip(i + 1) {
            if other == c {
                return false;
            }
        }
    }
    true
}
