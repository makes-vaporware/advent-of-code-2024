use itertools::Itertools;
// use std::fs::OpenOptions;
// use std::io::Write;

pub fn solve() {
    let input = crate::helpers::read_file("14A");
    let robots = input
        .lines()
        .map(|l| {
            l.chars()
                .filter(|&c| c.is_ascii_digit() || c == '-' || c == ',' || c == ' ')
                .collect::<String>()
                .split([',', ' '])
                .map(|n| n.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let m = 101;
    let n = 103;
    let mut robots_part_1 = robots.clone();

    for _ in 0..100 {
        robots_part_1.iter_mut().for_each(|r| {
            let [x, y, i, j] = [r[0], r[1], r[2], r[3]];
            r[0] = (x + i + m) % m;
            r[1] = (y + j + n) % n;
        });
    }

    let mut robots_in_quadrants = [0, 0, 0, 0];
    let m_mid = m / 2;
    let n_mid = n / 2;

    for robot in robots_part_1 {
        let [x, y] = [robot[0], robot[1]];
        if x < m_mid && y < n_mid {
            robots_in_quadrants[0] += 1;
        } else if x > m_mid && y < n_mid {
            robots_in_quadrants[1] += 1;
        } else if x < m_mid && y > n_mid {
            robots_in_quadrants[2] += 1;
        } else if x > m_mid && y > n_mid {
            robots_in_quadrants[3] += 1;
        }
    }

    let part1 = robots_in_quadrants.iter().product::<i32>();

    println!("{part1}");

    /*
        // brute force part 2, why not?
        let mut output_file = OpenOptions::new()
            .append(true)
            .open("src\\outputs\\14O.txt")
            .unwrap();

        let mut robots_part_2 = robots.clone();

        for i in 1..=8000 {
            let mut grid = vec![vec![false; 101]; 103];
            robots_part_2.iter_mut().for_each(|r| {
                let [x, y, i, j] = [r[0], r[1], r[2], r[3]];
                r[0] = (x + i + m) % m;
                r[1] = (y + j + n) % n;
                grid[r[1] as usize][r[0] as usize] = true;
            });

            writeln!(output_file, "{i}").unwrap();
            (0..103).for_each(|a| {
                (0..101).for_each(|b| {
                    if grid[a][b] {
                        write!(output_file, "X").unwrap();
                    } else {
                        write!(output_file, ".").unwrap();
                    }
                });
                writeln!(output_file).unwrap();
            });
        }
    */
}
