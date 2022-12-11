use std::{collections::HashSet, fs};

// (x, y)
// type Pos = (i32, i32);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x: x, y: y }
    }
    fn add(&mut self, other: (i32, i32)) -> Pos {
        Pos {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

fn main() {
    let filename = "input.txt";
    let file = fs::read_to_string(filename).expect("Could not read file {file_path}");
    let motions = file.trim().split('\n').collect::<Vec<&str>>();

    println!("{:#?}", motions);

    let mut positions: Vec<Pos> = Vec::new();
    let mut visits: HashSet<Pos> = HashSet::new();

    for _ in 0..10 {
        positions.push(Pos::new(0, 0));
    }
    visits.insert(positions.last().unwrap().clone());

    for motion in motions {
        let motion = motion.split(' ').collect::<Vec<&str>>();
        let dir = motion[0];
        let length = motion[1].parse::<i32>().unwrap();

        for _ in 0..length {
            match dir {
                "U" => positions[0] = positions[0].add((0, 1)),
                "L" => positions[0] = positions[0].add((-1, 0)),
                "R" => positions[0] = positions[0].add((1, 0)),
                "D" => positions[0] = positions[0].add((0, -1)),
                _ => println!("Error"),
            }

            for i in 1..positions.len() {
                let dist = distance(&positions[i - 1], &positions[i]);
                let x_dist = dist.0.abs();
                let y_dist = dist.1.abs();

                let x_close = x_dist <= 1;
                let y_close = y_dist <= 1;
                let same_col = x_dist == 0;
                let same_row = y_dist == 0;

                if x_close && y_close {
                    continue;
                }
                if !same_col {
                    if positions[i - 1].x > positions[i].x {
                        positions[i] = positions[i].add((1, 0));
                    } else {
                        positions[i] = positions[i].add((-1, 0));
                    }
                }
                if !same_row {
                    if positions[i - 1].y > positions[i].y {
                        positions[i] = positions[i].add((0, 1));
                    } else {
                        positions[i] = positions[i].add((0, -1));
                    }
                }

                visits.insert(positions.last().unwrap().clone());
            }
        }
    }

    println!("Unique visits: {}", visits.len());
}

fn distance(head: &Pos, tail: &Pos) -> (i32, i32) {
    (head.x - tail.x, head.y - tail.y)
}
