use itertools::Itertools;
use std::collections::HashMap;

pub fn solve() {
    let input = crate::helpers::read_file("5A");

    // Part 1 & 2
    let mut rules = HashMap::new();
    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.lines() {
        if line.contains("|") {
            let (parent, child) = line.split("|").collect_tuple().unwrap();
            let e = rules.entry(parent).or_insert(vec![]);
            e.push(child);
        } else if line.contains(",") {
            let mut pages = line.split(",").collect_vec();

            // logic was self-derived, but this method of conciseness was developed after reading this:
            // https://www.reddit.com/r/adventofcode/comments/1h71eyz/comment/m0hwrny/
            if pages.is_sorted_by(|a, b| !rules[b].contains(a)) {
                part1 += pages[pages.len() / 2].parse::<i32>().unwrap();
            } else {
                pages.sort_by(|a, b| rules[b].contains(a).cmp(&true));
                part2 += pages[pages.len() / 2].parse::<i32>().unwrap();
            }
        }
    }

    println!("{part1}");
    println!("{part2}");
}
