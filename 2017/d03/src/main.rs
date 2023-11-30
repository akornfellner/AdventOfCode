use std::collections::HashMap;

fn main() {
    println!("{}", solve_one(289326));
    println!("{}", solve_two(289326));
}

fn solve_one(input: i32) -> i32 {
    let (x, y) = get_coord(input);

    x.abs() + y.abs()
}

fn solve_two(input: i32) -> i32 {
    let a = (input as f64).sqrt().ceil() as usize;

    let mut seen: HashMap<(i32, i32), i32> = HashMap::new();

    seen.insert((0, 0), 1);

    let mut curr = (0, 0);
    let mut d = (1, 0);

    for c in 1..=a {
        for _ in 0..2 {
            for _ in 0..c {
                curr.0 += d.0;
                curr.1 += d.1;

                let mut value = 0;

                for neighbor in get_neighbors(curr) {
                    value += seen.get(&neighbor).unwrap_or(&0);
                }

                seen.insert(curr, value);

                if value > input {
                    return value;
                }
            }

            (d.0, d.1) = (d.1, -d.0);
        }
    }

    0
}

fn get_neighbors((x, y): (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (x, y - 1),
        (x, y + 1),
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ]
}

fn get_coord(input: i32) -> (i32, i32) {
    let a = (input as f64).sqrt().ceil() as usize;

    let mut curr = (0, 0);
    let mut d = (1, 0);

    let mut value = 1;

    for c in 1..=a {
        for _ in 0..2 {
            for _ in 0..c {
                curr.0 += d.0;
                curr.1 += d.1;

                value += 1;

                if value == input {
                    return curr;
                }
            }

            (d.0, d.1) = (d.1, -d.0);
        }
    }
    (0, 0)
}
