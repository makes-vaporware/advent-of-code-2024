use itertools::Itertools;
use std::collections::HashMap;

pub fn solve() {
    let input = crate::helpers::read_file("11A");
    let stones = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect_vec();

    // given stone n, returns number of stones after x blinks
    fn num_stones(stone: i64, blinks: i32, memo_map: &mut HashMap<(i64, i32), i64>) -> i64 {
        let result: i64;

        if let Some(v) = memo_map.get(&(stone, blinks)) {
            result = *v;
        } else if blinks == 0 {
            result = 1;
        } else if stone == 0 {
            result = num_stones(1, blinks - 1, memo_map);
        } else if stone.to_string().len() % 2 == 0 {
            let pow_ten = 10_i64.pow(stone.to_string().len() as u32 / 2);
            result = num_stones(stone / pow_ten, blinks - 1, memo_map)
                + num_stones(stone % pow_ten, blinks - 1, memo_map);
        } else {
            result = num_stones(stone * 2024, blinks - 1, memo_map);
        }

        memo_map.insert((stone, blinks), result);
        result
    }

    let mut memo_map = HashMap::new();

    // Part 1
    let mut part1 = 0;
    for stone in &stones {
        part1 += num_stones(*stone, 25, &mut memo_map);
    }
    println!("{}", part1);

    // Part 2
    let mut part2 = 0;
    for stone in &stones {
        part2 += num_stones(*stone, 75, &mut memo_map);
    }
    println!("{}", part2);
}
