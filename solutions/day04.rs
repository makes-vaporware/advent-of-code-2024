use itertools::Itertools;

pub fn solve() {
    let input = crate::helpers::read_file("4A");
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    // Part 1
    // www.geeksforgeeks.org/search-a-word-in-a-2d-grid-of-characters/
    fn count_word_search(word_str: &str, grid: &[Vec<char>], row: i32, col: i32) -> i32 {
        let mut count = 0;
        let x = [-1, -1, -1, 0, 0, 0, 1, 1, 1];
        let y = [-1, 0, 1, -1, 0, 1, -1, 0, 1];
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let len = word_str.len();
        let word = word_str.chars().collect_vec();

        if grid[row as usize][col as usize] != word[0] {
            return 0;
        }

        for dir in 0..9 {
            let mut curr_x = row + x[dir];
            let mut curr_y = col + y[dir];
            let mut k = 1;

            while k < len {
                if curr_x >= m || curr_x < 0 || curr_y >= n || curr_y < 0 {
                    break;
                }

                if grid[curr_x as usize][curr_y as usize] != word[k] {
                    break;
                }

                curr_x += x[dir];
                curr_y += y[dir];
                k += 1;
            }

            if k == len {
                count += 1;
            }
        }
        count
    }

    // Part 2
    fn x_mas_search(grid: &[Vec<char>], row: i32, col: i32) -> i32 {
        let x = [-1, -1, 1, 1];
        let y = [-1, 1, -1, 1];
        let c = [
            vec!['M', 'M', 'S', 'S'],
            vec!['S', 'M', 'S', 'M'],
            vec!['S', 'S', 'M', 'M'],
            vec!['M', 'S', 'M', 'S'],
        ];
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;

        if grid[row as usize][col as usize] != 'A' {
            return 0;
        }

        let mut chars = vec![];

        for dir in 0..4 {
            let curr_x = row + x[dir];
            let curr_y = col + y[dir];

            if curr_x >= m || curr_x < 0 || curr_y >= n || curr_y < 0 {
                return 0;
            }

            chars.push(grid[curr_x as usize][curr_y as usize]);
        }

        c.contains(&chars) as i32
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            part1 += count_word_search("XMAS", &grid, row as i32, col as i32);
            part2 += x_mas_search(&grid, row as i32, col as i32);
        }
    }

    println!("{part1}");
    println!("{part2}");
}
