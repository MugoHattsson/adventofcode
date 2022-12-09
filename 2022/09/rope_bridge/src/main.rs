use std::{collections::HashSet, fs};

// (x, y)
// type Pos = (i32, i32);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x: x, y: y }
    }
    fn add(&mut self, other: (i32, i32)) {
        self.x += other.0;
        self.y += other.1;
    }
}

fn main() {
    let filename = "input.txt";
    let file = fs::read_to_string(filename).expect("Could not read file {file_path}");
    let motions = file.trim().split('\n').collect::<Vec<&str>>();

    println!("{:#?}", motions);

    let mut head: Pos = Pos::new(0, 0);
    let mut tail: Pos = Pos::new(0, 0);
    let mut visits: HashSet<Pos> = HashSet::new();

    visits.insert(tail.clone());

    println!("{:?}", distance(&head, &tail));

    for motion in motions {
        let motion = motion.split(' ').collect::<Vec<&str>>();
        let dir = motion[0];
        let length = motion[1].parse::<i32>().unwrap();

        for _ in 0..length {
            match dir {
                "U" => head.add((0, 1)),
                "L" => head.add((-1, 0)),
                "R" => head.add((1, 0)),
                "D" => head.add((0, -1)),
                _ => println!("Error"),
            }

            let dist = distance(&head, &tail);
            match dist {
                (0, 0) => continue,
                (x, 0) => {
                    if x > 1 {
                        tail.add((x - 1, 0));
                    } else if x < -1 {
                        tail.add((x + 1, 0));
                    }
                }
                (0, y) => {
                    if y > 1 {
                        tail.add((0, y - 1));
                    } else if y < -1 {
                        tail.add((0, y + 1));
                    }
                }
                (x, y) => {
                    if x.abs() == 1 && y.abs() > 1 {
                        if y > 1 {
                            tail.add((x, y - 1));
                        } else {
                            tail.add((x, y + 1));
                        }
                    } else if x.abs() > 1 && y.abs() == 1 {
                        if x > 1 {
                            tail.add((x - 1, y));
                        } else {
                            tail.add((x + 1, y));
                        }
                    }
                }
            }

            visits.insert(tail.clone());
        }
    }

    println!("Unique visits: {}", visits.len());
}

fn distance(head: &Pos, tail: &Pos) -> (i32, i32) {
    (head.x - tail.x, head.y - tail.y)
}
