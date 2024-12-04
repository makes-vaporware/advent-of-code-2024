use itertools::Itertools;

fn safe_difference(n1: i32, n2: i32) -> bool {
    1 <= n1.abs_diff(n2) && n1.abs_diff(n2) <= 3
}

pub fn solve() {
    let input = crate::helpers::read_file("2A");

    // Part 1 & 2
    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.lines() {
        let levels = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect_vec();
        if (levels.is_sorted() || levels.is_sorted_by(|a, b| a > b))
            && levels.windows(2).all(|w| safe_difference(w[0], w[1]))
        {
            part1 += 1;
            part2 += 1;
        } else {
            for i in 0..levels.len() {
                let mut truncated_levels = levels.clone();
                truncated_levels.remove(i);
                if (truncated_levels.is_sorted() || truncated_levels.is_sorted_by(|a, b| a > b))
                    && truncated_levels
                        .windows(2)
                        .all(|w| safe_difference(w[0], w[1]))
                {
                    part2 += 1;
                    break;
                }
            }
        }
    }

    println!("{part1}");
    println!("{part2}");
}
