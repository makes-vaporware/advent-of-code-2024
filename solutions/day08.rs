use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve() {
    let input = crate::helpers::read_file("8A");
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut antennas = HashMap::new();
    let mut antinodes_part_1 = HashSet::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '.' {
                let antenna = antennas.entry(grid[i][j]).or_insert(vec![]);
                antenna.push((i as i32, j as i32));
            }
        }
    }

    // Part 1
    for positions in antennas.values() {
        for pair in positions.iter().combinations(2) {
            let diff = (pair[1].0 - pair[0].0, pair[1].1 - pair[0].1);
            let antinode_1 = (pair[0].0 - diff.0, pair[0].1 - diff.1);
            let antinode_2 = (pair[1].0 + diff.0, pair[1].1 + diff.1);

            if antinode_1.0 >= 0
                && antinode_1.0 < grid.len() as i32
                && antinode_1.1 >= 0
                && antinode_1.1 < grid[0].len() as i32
            {
                antinodes_part_1.insert(antinode_1);
            }

            if antinode_2.0 >= 0
                && antinode_2.0 < grid.len() as i32
                && antinode_2.1 >= 0
                && antinode_2.1 < grid[0].len() as i32
            {
                antinodes_part_1.insert(antinode_2);
            }
        }
    }

    let part1 = antinodes_part_1.len();
    println!("{part1}");

    // Part 2
    let mut antinodes_part_2 = HashSet::new();

    for positions in antennas.values() {
        for pair in positions.iter().combinations(2) {
            let diff = (pair[1].0 - pair[0].0, pair[1].1 - pair[0].1);
            let mut antinode_1 = *pair[0];
            let mut antinode_2 = *pair[1];

            while antinode_1.0 >= 0
                && antinode_1.0 < grid.len() as i32
                && antinode_1.1 >= 0
                && antinode_1.1 < grid[0].len() as i32
            {
                antinodes_part_2.insert(antinode_1);
                antinode_1.0 -= diff.0;
                antinode_1.1 -= diff.1;
            }

            while antinode_2.0 >= 0
                && antinode_2.0 < grid.len() as i32
                && antinode_2.1 >= 0
                && antinode_2.1 < grid[0].len() as i32
            {
                antinodes_part_2.insert(antinode_2);
                antinode_2.0 += diff.0;
                antinode_2.1 += diff.1;
            }
        }
    }

    let part2 = antinodes_part_2.len();
    println!("{part2}");
}
