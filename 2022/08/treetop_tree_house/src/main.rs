use std::fs;

type Grid = Vec<Vec<u32>>;

fn main() {
    let filename = "small.txt";
    let file = fs::read_to_string(filename).expect("Could not read file {file_path}");
    let lines = file.trim().split('\n').collect::<Vec<&str>>();

    let grid: Grid = lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    print_grid(&grid);

    let width: usize = grid[0].len();
    let height: usize = grid.len();

    let outline_count = width * 2 + (height - 2) * 2;

    println!("{outline_count}");

    let mut sum = 0;
    for row in 0..height {
        for col in 0..width {
            if visible(&grid, row, col) {
                sum += 1;
            }
            scenic_score(&grid, row, col);
        }
    }

    println!("{sum}");

    println!(
        "{:?}",
        grid[0..2]
            .iter()
            .map(|row| row[3])
            .rev()
            .collect::<Vec<u32>>()
    );
}

fn scenic_score(grid: &Grid, row: usize, col: usize) {
    let tree_height = grid[row][col];

    let left_view = grid[row][0..col]
        .iter()
        .rev()
        .map(|t| (*t < tree_height) as u32)
        .collect::<Vec<u32>>();

    let score = match left_view[..] {
        [] => 0,
        [_] => 1,
        [0, ..] => 1,
        _ => left_view
            .iter()
            .filter(|x| **x == 1)
            .collect::<Vec<&u32>>()
            .len(),
    };

    print!("Score: {score}  ");
    println!("{:?}", left_view);
}

fn visible(grid: &Grid, row: usize, col: usize) -> bool {
    let tree_height = grid[row][col];

    if !grid[row][0..col].iter().all(|t| *t < tree_height) {
        if !grid[row][col + 1..].iter().all(|t| *t < tree_height) {
            if !grid[0..row]
                .iter()
                .map(|row| row[col])
                .all(|t| t < tree_height)
            {
                if !grid[row + 1..]
                    .iter()
                    .map(|row| row[col])
                    .all(|t| t < tree_height)
                {
                    // println!("({row},{col}):  F");
                    return false;
                }
            }
        }
    }

    // println!("({row},{col}):  T");
    true
}

fn print_grid(grid: &Vec<Vec<u32>>) {
    for row in grid {
        println!("{:?}", row);
    }
}
