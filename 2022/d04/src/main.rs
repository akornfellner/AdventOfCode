use std::fs;

fn main() {
    println!("part one: {}", solve("input.txt", false));
    println!("part two: {}", solve("input.txt", true));
}

fn solve(input: &str, two: bool) -> usize {
    let input = fs::read_to_string(input).unwrap();
    let pairs: Vec<Vec<usize>> = input
        .lines()
        .map(|x| {
            x.replace(',', "-")
                .split('-')
                .map(|y| y.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    let mut count = 0usize;

    for pair in pairs {
        if !two
            && (pair[0] <= pair[2] && pair[1] >= pair[3]
                || pair[0] >= pair[2] && pair[1] <= pair[3])
        {
            count += 1;
        } else if two {
            let first: Vec<usize> = (pair[0]..=pair[1]).collect();
            let second: Vec<usize> = (pair[2]..=pair[3]).collect();

            if have_equal(&first, &second) {
                count += 1;
            }
        }
    }

    count
}

fn have_equal(first: &[usize], second: &[usize]) -> bool {
    let mut result = false;

    for value in first {
        if second.contains(value) {
            result = true;
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve("input_test.txt", false);
        assert_eq!(result, 2);
    }

    #[test]
    fn two_works() {
        let result = solve("input_test.txt", true);
        assert_eq!(result, 4);
    }
}
