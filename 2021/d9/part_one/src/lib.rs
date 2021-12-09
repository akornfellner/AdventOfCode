use std::fs;

pub fn solve(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).unwrap();

    let lines: Vec<&str> = input.split('\n').collect();

    let mut numbers: Vec<Vec<i32>> = vec![];

    for line in lines {
        let mut n: Vec<i32> = vec![];
        let chars: Vec<char> = line.chars().collect();
        for c in chars {
            n.push(c as i32 - 48);
        }
        numbers.push(n)
    }

    let mut mins: Vec<i32> = vec![];

    let rows = numbers.len();
    let cols = numbers[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if i > 0 && numbers[i][j] >= numbers[i - 1][j] {
                continue;
            }

            if j > 0 && numbers[i][j] >= numbers[i][j - 1] {
                continue;
            }

            if i < rows - 1 && numbers[i][j] >= numbers[i + 1][j] {
                continue;
            }

            if j < cols - 1 && numbers[i][j] >= numbers[i][j + 1] {
                continue;
            }

            mins.push(numbers[i][j])
        }
    }

    let mut sum: i32 = mins.iter().sum();
    sum += mins.len() as i32;
    sum
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt"), 15);
    }
}
