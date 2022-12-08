use std::fs;

type Grid = Vec<Vec<u32>>;

fn main() {
    let filename = "input.txt";
    let file = fs::read_to_string(filename).expect("Could not read file {file_path}");
    let lines = file.trim().split('\n').collect::<Vec<&str>>();

    let grid: Grid = lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let width: usize = grid[0].len();
    let height: usize = grid.len();

    let mut num_visible = 0;
    let mut scenic_scores: Vec<usize> = Vec::new();
    for row in 0..height {
        for col in 0..width {
            if visible(&grid, row, col) {
                num_visible += 1;
            }
            scenic_scores.push(scenic_score(&grid, row, col));
        }
    }

    println!("Visible trees:    {num_visible}");
    println!("Max scenic score: {}", scenic_scores.iter().max().unwrap());
}

fn scenic_score(grid: &Grid, row: usize, col: usize) -> usize {
    let tree_height = grid[row][col];

    let mut directions: Vec<Vec<u32>> = Vec::new();
    let mut scores = Vec::new();

    // Up
    directions.push(
        grid[0..row]
            .iter()
            .map(|row| row[col])
            .rev()
            .collect::<Vec<u32>>(),
    );
    // Left
    directions.push(
        grid[row][0..col]
            .iter()
            .rev()
            .map(|t| *t)
            .collect::<Vec<u32>>(),
    );
    // Right
    directions.push(
        grid[row][col + 1..]
            .iter()
            .map(|t| *t)
            .collect::<Vec<u32>>(),
    );
    // Down
    directions.push(
        grid[row + 1..]
            .iter()
            .map(|row| row[col])
            .collect::<Vec<u32>>(),
    );

    for dir in &directions {
        if dir.is_empty() {
            scores.push(0);
            continue;
        }

        let mut i: usize = 0;
        while i < dir.len() {
            if dir[i] >= tree_height {
                i += 1;
                break;
            }

            i += 1;
        }
        scores.push(i);
    }

    scores.iter().product::<usize>()
}

fn visible(grid: &Grid, row: usize, col: usize) -> bool {
    let tree_height = grid[row][col];

    // Up
    if grid[0..row]
        .iter()
        .map(|row| row[col])
        .all(|t| t < tree_height)
    {
        return true;
    }
    // Left
    if grid[row][0..col].iter().all(|t| *t < tree_height) {
        return true;
    }
    // Right
    if grid[row][col + 1..].iter().all(|t| *t < tree_height) {
        return true;
    }
    // Down
    if grid[row + 1..]
        .iter()
        .map(|row| row[col])
        .all(|t| t < tree_height)
    {
        return true;
    }

    false
}
