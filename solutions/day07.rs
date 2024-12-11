use itertools::Itertools;

pub fn solve() {
    let input = crate::helpers::read_file("7A");

    // Part 1
    fn mathificator_part_1(target_score: i64, current_score: i64, num_arr: &[i64]) -> bool {
        if num_arr.is_empty() {
            return target_score == current_score;
        }

        let n = num_arr[0];
        let n_mult = current_score * n;
        let n_add = current_score + n;
        let mut n_mult_res = false;
        let mut n_add_res = false;

        if n_mult > target_score && n_add > target_score {
            return false;
        }

        if n_mult <= target_score {
            n_mult_res = mathificator_part_1(target_score, n_mult, &num_arr[1..]);
        }

        if n_add <= target_score {
            n_add_res = mathificator_part_1(target_score, n_add, &num_arr[1..]);
        }

        n_mult_res || n_add_res
    }

    // Part 2
    fn mathificator_part_2(target_score: i64, current_score: i64, num_arr: &[i64]) -> bool {
        if num_arr.is_empty() {
            return target_score == current_score;
        }

        let n = num_arr[0];
        let n_mult = current_score * n;
        let n_add = current_score + n;
        let n_concat = (current_score.to_string() + &n.to_string())
            .parse::<i64>()
            .unwrap();
        let mut n_mult_res = false;
        let mut n_add_res = false;
        let mut n_concat_res = false;

        if n_mult > target_score && n_add > target_score && n_concat > target_score {
            return false;
        }

        if n_mult <= target_score {
            n_mult_res = mathificator_part_2(target_score, n_mult, &num_arr[1..]);
        }

        if n_add <= target_score {
            n_add_res = mathificator_part_2(target_score, n_add, &num_arr[1..]);
        }

        if n_concat <= target_score {
            n_concat_res = mathificator_part_2(target_score, n_concat, &num_arr[1..]);
        }

        n_mult_res || n_add_res || n_concat_res
    }

    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.lines() {
        let numbers = line
            .replace(":", "")
            .split_ascii_whitespace()
            .collect_vec()
            .into_iter()
            .map(|n| n.parse::<i64>().unwrap())
            .collect_vec();

        if mathificator_part_1(numbers[0], numbers[1], &numbers[2..]) {
            part1 += numbers[0];
        }

        if mathificator_part_2(numbers[0], numbers[1], &numbers[2..]) {
            part2 += numbers[0];
        }
    }

    println!("{part1}");
    println!("{part2}");
}
