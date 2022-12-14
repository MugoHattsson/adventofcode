use std::{collections::VecDeque, fs};

fn find_pos(heights: &[Vec<char>], c: char) -> (usize, usize) {
    *heights
        .iter()
        .enumerate()
        .map(|(i, row)| (i, row.iter().position(|x| *x == c)))
        .filter(|(_, o)| o.is_some())
        .map(|(i, o)| (i, o.unwrap()))
        .collect::<Vec<_>>()
        .first()
        .unwrap()
}

fn manhattan_dist(x: (i32, i32), y: (i32, i32)) -> u32 {
    x.0.abs_diff(y.0) + x.1.abs_diff(y.1)
}

fn neighbours(pos: (usize, usize), num_rows: usize, num_cols: usize) -> Vec<(usize, usize)> {
    let row = pos.0 as i32;
    let col = pos.1 as i32;

    vec![
        (row, col - 1),
        (row, col + 1),
        (row - 1, col),
        (row + 1, col),
    ]
    .iter()
    .filter(|x| manhattan_dist(**x, (row, col)) == 1)
    .filter(|(r, _)| 0 <= *r && *r < num_rows as i32)
    .filter(|(_, c)| 0 <= *c && *c < num_cols as i32)
    .copied()
    .map(|(r, c)| (r as usize, c as usize))
    .collect()
}

fn path_steps(pred: &[Vec<Option<(usize, usize)>>], end: (usize, usize)) -> usize {
    let mut pos = pred[end.0][end.1];
    let mut count = 0;
    while let Some((r, c)) = pos {
        pos = pred[r][c];
        count += 1;
    }
    count
}

fn bfs(heights: &[Vec<u8>], start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let num_rows: usize = heights.len();
    let num_cols: usize = heights[0].len();

    let mut visited = vec![vec![false; num_cols]; num_rows];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut pred = vec![vec![None; num_cols]; num_rows];

    visited[start.0][start.1] = true;
    queue.push_back(start);

    while !queue.is_empty() {
        let current: (usize, usize) = queue.pop_front().unwrap();
        let c_height = heights[current.0][current.1];

        for neighbour in neighbours(current, num_rows, num_cols)
            .iter()
            .filter(|(nr, nc)| heights[*nr][*nc] <= c_height + 1)
        {
            if !visited[neighbour.0][neighbour.1] {
                visited[neighbour.0][neighbour.1] = true;
                queue.push_back(*neighbour);
                pred[neighbour.0][neighbour.1] = Some(current);

                if *neighbour == end {
                    return Some(path_steps(&pred, end));
                }
            }
        }
    }
    None
}

fn main() {
    let filename = "input.txt";
    let file = fs::read_to_string(filename).expect("Could not read file {file_path}");
    let input = file.trim().split('\n').collect::<Vec<&str>>();

    let mut heights = input
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let s = find_pos(&heights, 'S');
    let end = find_pos(&heights, 'E');

    heights[s.0][s.1] = 'a';
    heights[end.0][end.1] = 'z';
    let heights: Vec<Vec<_>> = heights
        .iter()
        .map(|row| row.iter().map(|c| *c as u8 - 97).collect())
        .collect();

    let num_rows: usize = heights.len();
    let num_cols: usize = heights[0].len();

    let mut paths = Vec::new();
    (0..num_rows).for_each(|row| {
        (0..num_cols).for_each(|col| {
            if heights[row][col] == 0 {
                paths.push(bfs(&heights, (row, col), end));
            }
        })
    });

    println!(
        "Shortest path from any start at elevation a: {:?}",
        paths
            .iter()
            .filter(|p| p.is_some())
            .flatten()
            .min()
            .unwrap()
    );
}
