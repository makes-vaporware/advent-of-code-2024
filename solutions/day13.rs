use itertools::Itertools;

pub fn solve() {
    let input = crate::helpers::read_file("13A");
    let machines = input
        .lines()
        .collect_vec()
        .chunks(4)
        .map(|x| x.to_vec())
        .collect_vec();

    // kermit: use regex, it'll look sleeker
    // evil kermit: standard library go brr
    fn x_y_extractor(str: &str) -> Vec<i64> {
        str.chars()
            .filter(|&c| c.is_ascii_digit() || c == ',')
            .collect::<String>()
            .split(",")
            .map(|n| n.parse::<i64>().unwrap())
            .collect_vec()
    }

    let mut part1 = 0;
    let mut part2 = 0;

    for machine in machines {
        let button_a = x_y_extractor(machine[0]);
        let button_b = x_y_extractor(machine[1]);
        let prize = x_y_extractor(machine[2]);
        let prize_2 = [prize[0] + 10000000000000, prize[1] + 10000000000000];

        let det_a = button_a[0] * button_b[1] - button_a[1] * button_b[0];
        let det_a1 = prize[0] * button_b[1] - prize[1] * button_b[0];
        let det_a2 = button_a[0] * prize[1] - button_a[1] * prize[0];
        let det_b1 = prize_2[0] * button_b[1] - prize_2[1] * button_b[0];
        let det_b2 = button_a[0] * prize_2[1] - button_a[1] * prize_2[0];

        // filter out no solutions or non discrete solutions
        if det_a != 0 && det_a1 % det_a == 0 && det_a2 % det_a == 0 {
            part1 += 3 * (det_a1 / det_a) + (det_a2 / det_a);
        }

        if det_a != 0 && det_b1 % det_a == 0 && det_b2 % det_a == 0 {
            part2 += 3 * (det_b1 / det_a) + (det_b2 / det_a);
        }
    }

    println!("{part1}");
    println!("{part2}");
}
