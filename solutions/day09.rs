use itertools::Itertools;

pub fn solve() {
    let input = crate::helpers::read_file("9A");
    // Part 1
    // more abstract/straightforward approach to files (string representations)
    // create the fragmented disk
    let mut disk_part_1 = vec![];
    let mut file_id = 0;
    let mut load_file = true;

    for c in input.chars() {
        if load_file {
            disk_part_1.append(
                &mut (0..(c.to_digit(10).unwrap()))
                    .map(|_| file_id.to_string())
                    .collect_vec(),
            );
            file_id += 1;
            load_file = false;
        } else {
            disk_part_1.append(
                &mut (0..(c.to_digit(10).unwrap()))
                    .map(|_| ".".to_string())
                    .collect_vec(),
            );
            load_file = true;
        }
    }

    let mut start_index = 0_usize;
    let mut end_index = disk_part_1.len() - 1;

    // swap vec elements around until all file segments are grouped without spaces
    while start_index != end_index && start_index < end_index {
        while disk_part_1[end_index] == "." && end_index > 0 {
            if end_index == 0 {
                break;
            }
            end_index -= 1;
        }

        while disk_part_1[start_index] != "." && start_index < disk_part_1.len() {
            start_index += 1;
        }

        if start_index < end_index {
            disk_part_1[start_index] = disk_part_1[end_index].clone();
            disk_part_1[end_index] = ".".to_string();
        }
    }

    let mut part1 = 0;
    for (i, file_id) in disk_part_1.iter().enumerate() {
        if file_id == "." {
            break;
        }
        part1 += i as i64 * file_id.parse::<i64>().unwrap();
    }

    println!("{}", part1);

    // Part 2
    // more formal solution for storing files (struct representation)
    // this took 1765.76s ~= 30 minutes this is a HORRIBLE solution
    #[derive(Debug)]
    struct File {
        file_id: i32,
        start_index: u32,
        size: u32,
    }

    let mut disk_part_2 = vec![];
    let mut file_id = 0;
    let mut disk_index = 0;
    let mut load_file = true;

    for c in input.chars() {
        if load_file {
            disk_part_2.push(File {
                file_id,
                start_index: disk_index,
                size: c.to_digit(10).unwrap(),
            });
            disk_index += c.to_digit(10).unwrap();
            file_id += 1;
            load_file = false;
        } else {
            disk_index += c.to_digit(10).unwrap();
            load_file = true;
        }
    }

    let max_file_id = file_id;

    // swap files around in blocks instead of individual pieces
    for n in (0..max_file_id).rev() {
        for i in 0..max_file_id - 1 {
            let file_a_start_index = disk_part_2[i as usize].start_index;
            let file_a_size = disk_part_2[i as usize].size;
            let file_b_start_index = disk_part_2[(i + 1) as usize].start_index;
            let free_space = file_b_start_index - (file_a_start_index + file_a_size);
            let file_n = disk_part_2.iter_mut().find(|x| x.file_id == n).unwrap();

            if file_a_start_index >= file_n.start_index {
                break;
            }

            if free_space >= file_n.size {
                file_n.start_index = file_a_start_index + file_a_size;
                disk_part_2.sort_by(|a, b| a.start_index.cmp(&b.start_index));
                break;
            }
        }
    }

    let mut part2 = 0;
    for file in disk_part_2 {
        part2 += (file.start_index..file.start_index + file.size)
            .map(|n| n as i64 * file.file_id as i64)
            .sum::<i64>();
    }

    println!("{part2}");
}
