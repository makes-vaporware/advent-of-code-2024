use itertools::Itertools;

pub fn solve() {
    let input = crate::helpers::read_file("6A");
    let mut grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut grid_visited = vec![vec![false; grid[0].len()]; grid.len()];

    #[derive(PartialEq, Clone, Copy)]
    enum Direction {
        Up,
        Right,
        Down,
        Left,
    }

    // find guard
    let mut guard_pos_ori = (0, 0);
    let mut guard_pos = (0, 0);
    let mut guard_dir = Direction::Up;

    for (i, row) in grid.iter().enumerate() {
        if row.contains(&'^') {
            guard_pos_ori = (i as i32, row.iter().position(|&c| c == '^').unwrap() as i32);
            guard_pos = guard_pos_ori;
            grid_visited[guard_pos.0 as usize][guard_pos.1 as usize] = true;
        }
    }

    loop {
        let next_pos = {
            match guard_dir {
                Direction::Up => (guard_pos.0 - 1, guard_pos.1),
                Direction::Right => (guard_pos.0, guard_pos.1 + 1),
                Direction::Down => (guard_pos.0 + 1, guard_pos.1),
                Direction::Left => (guard_pos.0, guard_pos.1 - 1),
            }
        };

        // check for exiting mapped area
        if next_pos.0 < 0
            || next_pos.0 >= grid.len() as i32
            || next_pos.1 < 0
            || next_pos.1 >= grid[0].len() as i32
        {
            break;
        }

        // check for obstacles
        if grid[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            match guard_dir {
                Direction::Up => guard_dir = Direction::Right,
                Direction::Right => guard_dir = Direction::Down,
                Direction::Down => guard_dir = Direction::Left,
                Direction::Left => guard_dir = Direction::Up,
            }
        } else {
            guard_pos = next_pos;
            grid_visited[guard_pos.0 as usize][guard_pos.1 as usize] = true;
        }
    }

    let mut part1 = 0;
    for row in &grid_visited {
        part1 += row.iter().filter(|e| **e).count();
    }

    println!("{part1}");

    // Part 2
    let mut part2 = 0;

    // place obstacles along path that guard will travel in
    // loop detection is when reach same pos with same dir
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            // don't place obstacle on starting point
            if grid_visited[i][j] && (i as i32, j as i32) != guard_pos_ori {
                let mut grid_visited_advanced = (0..grid.len())
                    .map(|_| (0..grid[0].len()).map(|_| Vec::new()).collect_vec())
                    .collect_vec();
                let mut guard_pos = guard_pos_ori;
                let mut guard_dir = Direction::Up;

                // place obstacle there
                grid[i][j] = '#';

                // run simulation and check for loops
                loop {
                    let next_pos = {
                        match guard_dir {
                            Direction::Up => (guard_pos.0 - 1, guard_pos.1),
                            Direction::Right => (guard_pos.0, guard_pos.1 + 1),
                            Direction::Down => (guard_pos.0 + 1, guard_pos.1),
                            Direction::Left => (guard_pos.0, guard_pos.1 - 1),
                        }
                    };

                    // check for exiting mapped area - no loop.
                    if next_pos.0 < 0
                        || next_pos.0 >= grid.len() as i32
                        || next_pos.1 < 0
                        || next_pos.1 >= grid[0].len() as i32
                    {
                        break;
                    }

                    // loop detection
                    if grid_visited_advanced[next_pos.0 as usize][next_pos.1 as usize]
                        .contains(&guard_dir)
                    {
                        // has loop. break.
                        part2 += 1;
                        break;
                    }

                    // check for obstacles
                    if grid[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                        match guard_dir {
                            Direction::Up => guard_dir = Direction::Right,
                            Direction::Right => guard_dir = Direction::Down,
                            Direction::Down => guard_dir = Direction::Left,
                            Direction::Left => guard_dir = Direction::Up,
                        }
                    } else {
                        guard_pos = next_pos;
                        grid_visited_advanced[guard_pos.0 as usize][guard_pos.1 as usize]
                            .push(guard_dir);
                    }
                }

                // remove obstacle there
                grid[i][j] = '.';
            }
        }
    }

    println!("{part2}");
}
