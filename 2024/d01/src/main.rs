fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (i32, i32) {
    let input = std::fs::read_to_string(filename).unwrap();

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        left.push(parts[0].parse().unwrap());
        right.push(parts[1].parse().unwrap());
    });

    left.sort();
    right.sort();
    let mut sum = 0;

    for (i, left_value) in left.iter().enumerate() {
        let right_value = right[i];
        sum += (left_value - right_value).abs();
    }

    let mut sum2 = 0;

    for left_value in left {
        let mut count = 0;
        for right_value in &right {
            if left_value == *right_value {
                count += 1
            }
        }
        sum2 += count * left_value;
    }

    (sum, sum2)
}
