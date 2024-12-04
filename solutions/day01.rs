use itertools::Itertools;

pub fn solve() {
    let input = crate::helpers::read_file("1A");

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    left.sort();
    right.sort();

    // Part 1
    let mut part1 = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        part1 += (l - r).abs();
    }

    println!("{part1}");

    // Part 2
    let counts = right.into_iter().counts();
    let mut part2 = 0;
    for l in left.iter() {
        part2 += l * counts.get(l).copied().unwrap_or(0) as i32;
    }

    println!("{part2}");
}
