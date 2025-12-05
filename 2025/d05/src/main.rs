use std::env::args;
fn main() {
    let filename = args().nth(1).unwrap_or("input_test.txt".to_string());
    let (p1, p2) = solve(&filename);
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();

    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let ranges: Vec<(usize, usize)> = parts[0]
        .lines()
        .map(|line| {
            let nums = line.split('-').collect::<Vec<&str>>();
            (
                nums[0].parse::<usize>().unwrap(),
                nums[1].parse::<usize>().unwrap(),
            )
        })
        .collect();

    let ids: Vec<usize> = parts[1]
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    let mut p1 = 0;

    for id in ids.iter() {
        for (start, end) in ranges.iter() {
            if id >= start && id <= end {
                p1 += 1;
                break;
            }
        }
    }

    let mut p2 = 0;

    let mut visited = vec![];

    for (start, end) in ranges.iter() {
        let mut new_start = *start;
        let mut new_end = *end;
        let mut subtract = 0;
        for (v_start, v_end) in visited.iter() {
            if new_start >= *v_start && new_start <= *v_end {
                new_start = v_end + 1;
            }
            if new_end >= *v_start && new_end <= *v_end {
                new_end = v_start - 1;
            }
            if new_start <= *v_start && new_end >= *v_end {
                subtract += v_end - v_start + 1;
            }
        }
        if new_start <= new_end {
            p2 += new_end - new_start + 1 - subtract;
            visited.push((new_start, new_end));
        }
    }

    (p1, p2)
}
