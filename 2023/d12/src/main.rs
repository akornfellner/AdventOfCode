fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    let mut springs: Vec<&str> = vec![];
    let mut numbers: Vec<Vec<usize>> = vec![];

    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        springs.push(parts[0]);
        numbers.push(
            parts[1]
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        );
    }

    for i in 0..springs.len() {
        result.0 += possibilities(springs[i], &numbers[i]);
    }

    result
}
fn possibilities(springs: &str, numbers: &[usize]) -> usize {
    if springs.len() == 0 && numbers.len() == 0 {
        return 1;
    }
    if springs.len() == 0 {
        return 0;
    }

    if numbers.len() == 0 {
        if springs.contains("#") {
            return 0;
        } else {
            return 1;
        }
    }

    if numbers.iter().sum::<usize>() > springs.len() {
        return 0;
    }

    let first = springs.chars().nth(0).unwrap();

    match first {
        '.' => {
            let result = possibilities(&springs[1..], numbers);
            result
        }
        '#' => {
            let n = numbers[0];
            for i in 0..n {
                if springs.chars().nth(i).unwrap() == '.' {
                    return 0;
                }
            }

            if n > springs.len() {
                return 0;
            }
            if n == springs.len() {
                return 1;
            } else if springs.chars().nth(n).unwrap() == '#' {
                return 0;
            }
            possibilities(&springs[n + 1..], &numbers[1..])
        }
        _ => {
            let s1 = String::from("#") + &springs[1..];
            let s2 = String::from(".") + &springs[1..];
            let result = possibilities(&s1, &numbers) + possibilities(&s2, numbers);
            result
        }
    }
}
