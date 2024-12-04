use regex::Regex;

pub fn solve() {
    let input = crate::helpers::read_file("3A");

    // the real regex is r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)"
    // i've added two dummy capture groups to do() and don't() just to provide rust with a consistent number of capture groups
    let re = Regex::new(r"do\(\)()()|don't\(\)()()|mul\((\d+),(\d+)\)").unwrap();

    // Part 1 & 2
    let mut part1 = 0;
    let mut part2 = 0;
    let mut active = true;
    for line in input.lines() {
        for (instruction, parameters) in re.captures_iter(line).map(|c| c.extract()) {
            match instruction {
                "do()" => {
                    active = true;
                }
                "don't()" => {
                    active = false;
                }
                _ => {
                    let [n1, n2] = parameters;
                    let eval = n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap();
                    part1 += eval;
                    if active {
                        part2 += eval;
                    }
                }
            }
        }
    }

    println!("{part1}");
    println!("{part2}");
}
