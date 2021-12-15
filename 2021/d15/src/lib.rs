use std::fs;

pub fn solve(filename: &str, p2: bool) -> i64 {
    let input = fs::read_to_string(filename).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let mut numbers: Vec<Vec<i64>> = vec![];
    let mut risks: Vec<Vec<i64>> = vec![];

    for line in lines {
        let mut l: Vec<i64> = vec![];
        for c in line.chars().collect::<Vec<char>>() {
            l.push(c as i64 - 48);
        }
        numbers.push(l);
    }

    if p2 {
        numbers = expand_field(&numbers);
    }

    for line in &numbers {
        let mut l: Vec<i64> = vec![];
        for _ in line {
            l.push(std::i64::MAX);
        }
        risks.push(l);
    }

    risks[0][0] = 0;

    let mut todo = vec![(0usize, 0usize)];

    let rows = numbers.len() - 1;
    let cols = numbers[0].len() - 1;

    while !todo.is_empty() {
        let (i, j) = todo.pop().unwrap();
        let value = risks[i][j];

        if i > 0 {
            let (m, n) = (i - 1, j);
            let old = risks[m][n];
            let new = numbers[m][n] + value;
            if new < old {
                risks[m][n] = new;
                todo.push((m, n));
            }
        }
        if j > 0 {
            let (m, n) = (i, j - 1);
            let old = risks[m][n];
            let new = numbers[m][n] + value;
            if new < old {
                risks[m][n] = new;
                todo.push((m, n));
            }
        }
        if i < rows {
            let (m, n) = (i + 1, j);
            let old = risks[m][n];
            let new = numbers[m][n] + value;
            if new < old {
                risks[m][n] = new;
                todo.push((m, n));
            }
        }
        if j < cols {
            let (m, n) = (i, j + 1);
            let old = risks[m][n];
            let new = numbers[m][n] + value;
            if new < old {
                risks[m][n] = new;
                todo.push((m, n));
            }
        }
    }

    risks[rows][cols]
}

pub fn expand_field(numbers: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let mut expand: Vec<Vec<i64>> = vec![];

    for line in numbers {
        let mut l: Vec<i64> = vec![];
        for i in 0..5i64 {
            for number in line {
                l.push(reduce(number + i));
            }
        }
        expand.push(l);
    }

    let tmp = expand.clone();
    expand.clear();

    for i in 0..5i64 {
        for line in &tmp {
            let new: Vec<i64> = line.iter().map(|x| reduce(*x + i)).collect();
            expand.push(new);
        }
    }

    expand
}

fn reduce(mut x: i64) -> i64 {
    if x > 9 {
        x -= 9;
    }
    x
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn p1_works() {
        assert_eq!(solve("input_test.txt", false), 40);
    }

    #[test]
    fn p2_works() {
        assert_eq!(solve("input_test.txt", true), 315);
    }
}
