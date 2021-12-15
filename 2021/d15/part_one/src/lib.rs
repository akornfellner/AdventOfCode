use std::fs;

pub fn solve(filename: &str) -> i64 {
    let input = fs::read_to_string(filename).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let mut numbers: Vec<Vec<i64>> = vec![];
    let mut risks: Vec<Vec<i64>> = vec![];

    for line in lines {
        let mut l: Vec<i64> = vec![];
        let mut r: Vec<i64> = vec![];
        for c in line.chars().collect::<Vec<char>>() {
            l.push(c as i64 - 48);
            r.push(std::i64::MAX);
        }
        numbers.push(l);
        risks.push(r);
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

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt"), 40);
    }
}
