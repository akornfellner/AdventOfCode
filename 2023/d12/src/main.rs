use std::collections::HashMap;

fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    let mut springs1: Vec<&str> = vec![];
    let mut numbers1: Vec<Vec<usize>> = vec![];
    let mut springs2: Vec<String> = vec![];
    let mut numbers2: Vec<Vec<usize>> = vec![];

    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        springs1.push(parts[0]);
        let n = parts[1]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        numbers1.push(n.clone());

        let mut m = n.clone();

        let mut s = String::from(parts[0]);
        for _ in 0..4 {
            s.push('?');
            s += parts[0];
            m.extend(n.iter());
        }
        springs2.push(s);
        numbers2.push(m);
    }

    for i in 0..springs1.len() {
        let mut calculated: Calc = HashMap::new();
        result.0 += possibilities(springs1[i], &numbers1[i], &mut calculated);
        result.1 += possibilities(&springs2[i], &numbers2[i], &mut calculated);
    }

    result
}

type Calc = HashMap<(String, Vec<usize>), usize>;

fn possibilities(springs: &str, numbers: &[usize], calculated: &mut Calc) -> usize {
    let key = &(springs.to_string(), numbers.to_vec());
    if calculated.contains_key(key) {
        let result = *calculated.get(key).unwrap();
        return result;
    }
    if springs.is_empty() && numbers.is_empty() {
        return 1;
    }
    if springs.is_empty() {
        return 0;
    }

    if numbers.is_empty() {
        if springs.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }

    if numbers.iter().sum::<usize>() > springs.len() {
        return 0;
    }

    let first = springs.chars().next().unwrap();

    match first {
        '.' => {
            let result = possibilities(&springs[1..], numbers, calculated);
            calculated.insert(key.clone(), result);
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
            let result = possibilities(&springs[n + 1..], &numbers[1..], calculated);
            calculated.insert(key.clone(), result);
            result
        }
        _ => {
            let s1 = String::from("#") + &springs[1..];
            let s2 = String::from(".") + &springs[1..];
            let result =
                possibilities(&s1, numbers, calculated) + possibilities(&s2, numbers, calculated);
            calculated.insert(key.clone(), result);
            result
        }
    }
}
