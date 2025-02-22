use itertools::Itertools;
use std::collections::HashSet;

pub fn solve() {
    let input = crate::helpers::read_file("10A");
    let grid = input
        .lines()
        .map(|l| l.chars().map(|n| n.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    // Part 1 & 2
    fn find_score_or_rating(grid: &[Vec<u32>], row: i32, col: i32, part_1: bool) -> u32 {
        // dfs graph traversal to find a path from 0 to 9.
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let x = [-1, 0, 1, 0];
        let y = [0, 1, 0, -1];
        let mut tally = 0;
        let mut stack = vec![(row, col, 0)];
        let mut visited = HashSet::new();

        while let Some(cell) = stack.pop() {
            // for part 1: track visited cells as usual
            // for part 2: i'm blurry on the logic, but you basically allow backtracking if
            // you're trying to find all possible paths to an exit
            // algo won't get stuck in a loop because directed graphs will force you in one direction
            // (the intuition that it might get stuck in a loop only applies to undirected graphs)
            if part_1 {
                visited.insert(cell);
            }

            if cell.2 == 9 {
                tally += 1;
                continue;
            }

            for dir in 0..4 {
                let adj_x = cell.0 + x[dir];
                let adj_y = cell.1 + y[dir];
                let adj_height = cell.2 + 1;

                // check it's in bounds, check it's the right height, check it's unvisited.
                if (0..m).contains(&adj_x)
                    && (0..n).contains(&adj_y)
                    && grid[adj_x as usize][adj_y as usize] == adj_height
                    && !visited.contains(&(adj_x, adj_y, adj_height))
                {
                    stack.push((adj_x, adj_y, adj_height));
                }
            }
        }

        tally
    }

    let mut part1 = 0;
    let mut part2 = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 0 {
                part1 += find_score_or_rating(&grid, row as i32, col as i32, true);
                part2 += find_score_or_rating(&grid, row as i32, col as i32, false);
            }
        }
    }

    println!("{part1}");
    println!("{part2}");
}
