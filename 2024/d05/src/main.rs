fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();

    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let mut rules: Vec<(usize, usize)> = vec![];

    for line in parts[0].lines() {
        let splitted = line.split("|").collect::<Vec<&str>>();
        rules.push((splitted[0].parse().unwrap(), splitted[1].parse().unwrap()));
    }

    let updates: Vec<Vec<usize>> = parts[1]
        .lines()
        .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut p1 = 0;
    let mut p2 = 0;

    for update in &updates {
        let relevant_rules = get_rules(&rules, update);
        if check_rules(&relevant_rules, update) {
            let middle = update[update.len() / 2];
            p1 += middle;
        } else {
            let sorted = sort_upated(update, &relevant_rules);
            let middle = sorted[sorted.len() / 2];
            p2 += middle;
        }
    }

    (p1, p2)
}

fn get_rules(rules: &[(usize, usize)], update: &[usize]) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for (x, y) in rules {
        if update.contains(x) && update.contains(y) {
            result.push((*x, *y));
        }
    }
    result
}

fn check_rules(rules: &[(usize, usize)], update: &[usize]) -> bool {
    for i in 1..update.len() {
        for j in 0..i {
            for (x, y) in rules {
                if update[i] == *x && update[j] == *y {
                    return false;
                }
            }
        }
    }
    true
}

fn sort_upated(update: &[usize], rules: &[(usize, usize)]) -> Vec<usize> {
    let mut result = vec![];
    let mut rest = update.to_vec();
    let mut rest_rules = rules.to_vec();

    while !rest.is_empty() {
        let x = rest.remove(0);
        let mut found = true;

        for (_, right) in &rest_rules {
            if x == *right {
                rest.push(x);
                found = false;
                break;
            }
        }

        if found {
            result.push(x);
            rest_rules.retain(|(left, _)| *left != x)
        }
    }

    result
}
