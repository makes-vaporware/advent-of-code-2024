use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub fn solve() {
    let input = crate::helpers::read_file("12A");
    let mut grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let m = grid.len();
    let n = grid[0].len();

    // BFS to find all of same crop_type
    fn price(grid: &mut [Vec<char>], row: i32, col: i32) -> (i32, i32, i32) {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let x = [-1, 0, 1, 0];
        let y = [0, 1, 0, -1];
        let crop_type = grid[row as usize][col as usize];
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((row, col));

        while let Some(cell) = queue.pop_front() {
            visited.insert(cell);

            for dir in 0..4 {
                let adj_x = cell.0 + x[dir];
                let adj_y = cell.1 + y[dir];

                // check it's in bounds, check it's the right crop, check it's unvisited.
                if (0..m).contains(&adj_x)
                    && (0..n).contains(&adj_y)
                    && grid[adj_x as usize][adj_y as usize] == crop_type
                    && !visited.contains(&(adj_x, adj_y))
                {
                    queue.push_back((adj_x, adj_y));
                    visited.insert((adj_x, adj_y));
                }
            }
        }

        let area = visited.len() as i32;
        let mut perimeter = 0;
        let mut corners = 0;

        // count for part 1
        for cell in &visited {
            for dir in 0..4 {
                let adj_x = cell.0 + x[dir];
                let adj_y = cell.1 + y[dir];

                if !visited.contains(&(adj_x, adj_y)) {
                    perimeter += 1;
                }
            }

            // count for part 2
            // corners = sides. how to find exterior/interior corners?
            // https://www.reddit.com/r/adventofcode/comments/1hcdnk0/comment/m1nkmol/
            for dir in 0..4 {
                let adj_x = cell.0 + x[dir];
                let adj_y = cell.1 + y[dir];
                let adj_x_2 = cell.0 + x[(dir + 1) % 4];
                let adj_y_2 = cell.1 + y[(dir + 1) % 4];
                let adj_x_c = cell.0 + x[dir] + x[(dir + 1) % 4];
                let adj_y_c = cell.1 + y[dir] + y[(dir + 1) % 4];

                // exterior corner || interior corner
                if (!visited.contains(&(adj_x, adj_y)) && !visited.contains(&(adj_x_2, adj_y_2)))
                    || (visited.contains(&(adj_x, adj_y))
                        && visited.contains(&(adj_x_2, adj_y_2))
                        && !visited.contains(&(adj_x_c, adj_y_c)))
                {
                    corners += 1;
                }
            }

            // mark as globally visited
            grid[cell.0 as usize][cell.1 as usize] = '.';
        }

        (area, perimeter, corners)
    }

    let mut part1 = 0;
    let mut part2 = 0;

    for row in 0..m {
        for col in 0..n {
            if grid[row][col] != '.' {
                let (area, perimeter, sides) = price(&mut grid, row as i32, col as i32);
                part1 += area * perimeter;
                part2 += area * sides;
            }
        }
    }

    println!("{part1}");
    println!("{part2}");
}
