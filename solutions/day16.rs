use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

type DirectedCoordinate = ((i32, i32), Direction);

pub fn solve() {
    let input = crate::helpers::read_file("16A");
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    // find start and end points of maze
    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                start = (i as i32, j as i32);
            }
            if grid[i][j] == 'E' {
                end = (i as i32, j as i32);
            }
        }
    }

    fn clockwise(dir: &Direction) -> Direction {
        match dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn anticlockwise(dir: &Direction) -> Direction {
        match dir {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn step_coord((x, y): &(i32, i32), dir: &Direction) -> (i32, i32) {
        match dir {
            Direction::North => (x - 1, *y),
            Direction::East => (*x, y + 1),
            Direction::South => (x + 1, *y),
            Direction::West => (*x, y - 1),
        }
    }

    fn foo(grid: Vec<Vec<char>>, start: (i32, i32), end: (i32, i32)) -> (i32, usize) {
        let mut heap = BinaryHeap::new(); // priority queue to get lowest cost node
        let mut dist = HashMap::new(); // distance
        let mut prev: HashMap<DirectedCoordinate, Vec<DirectedCoordinate>> = HashMap::new(); // keep track of parent
        let mut end_score = -1;
        let mut best_tiles: HashSet<(i32, i32)> = HashSet::new();
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;

        dist.insert((start, Direction::East), 0);
        heap.push((Reverse(0), start, Direction::East));

        // while heap is not empty, get the vertex in heap that has the min cost.
        while let Some((Reverse(score), node, dir)) = heap.pop() {
            if end_score != -1 && score > end_score {
                break;
            }

            if node == end {
                end_score = score;

                fn track_best_tiles(
                    prev: &mut HashMap<DirectedCoordinate, Vec<DirectedCoordinate>>,
                    best_tiles: &mut HashSet<(i32, i32)>,
                    n: ((i32, i32), Direction),
                ) {
                    if let Some(parents) = prev.get(&n).cloned() {
                        for (curr_node, curr_dir) in parents {
                            best_tiles.insert(curr_node);
                            track_best_tiles(prev, best_tiles, (curr_node, curr_dir));
                        }
                    }
                }

                best_tiles.insert(end);
                track_best_tiles(&mut prev, &mut best_tiles, (node, dir));
            }

            let adj_nodes = [
                (step_coord(&node, &clockwise(&dir)), clockwise(&dir)),
                (step_coord(&node, &dir), dir),
                (step_coord(&node, &anticlockwise(&dir)), anticlockwise(&dir)),
            ];

            for ((adj_x, adj_y), adj_dir) in adj_nodes {
                if (0..m).contains(&adj_x)
                    && (0..n).contains(&adj_y)
                    && grid[adj_x as usize][adj_y as usize] != '#'
                {
                    // dijkstra: update lowest distance if needed.
                    let path_cost: i32 = if dir != adj_dir { 1001 } else { 1 };

                    match dist
                        .get(&((adj_x, adj_y), adj_dir))
                        .unwrap_or(&i32::MAX)
                        .cmp(&(score + path_cost))
                    {
                        std::cmp::Ordering::Greater => {
                            // if new path is better, replace current parent nodes
                            dist.insert(((adj_x, adj_y), adj_dir), score + path_cost);
                            heap.push((Reverse(score + path_cost), (adj_x, adj_y), adj_dir));
                            // prev.insert(((adj_x, adj_y), adj_dir), vec![(node, dir)]);
                            prev.insert(((adj_x, adj_y), adj_dir), vec![(node, dir)]);
                        }
                        std::cmp::Ordering::Equal => {
                            // if new path is equal, add node to list of parents
                            prev.entry(((adj_x, adj_y), adj_dir))
                                .or_default()
                                .push((node, dir));
                        }
                        std::cmp::Ordering::Less => {
                            // if new path is worse, do nothing
                        }
                    }
                }
            }
        }

        for (a, b) in prev {
            if b.len() > 1 {
                println!("{:?}: {:?}", a, b);
            }
        }
        (end_score, best_tiles.len())
    }

    // Part 1 & 2
    let (part1, part2) = foo(grid, start, end);
    println!("{part1} {part2}");
}
