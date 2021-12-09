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
            let mut starti = -1;
            let mut endi = 1;
            let mut startj = -1;
            let mut endj = 1;

            if i == 0 {
                starti = 0;
            } else if i == rows - 1 {
                endi = 0;
            }

            if j == 0 {
                startj = 0;
            } else if j == cols - 1 {
                endj = 0;
            }

            let mut is_min = true;

            for k in starti..endi + 1 {
                let index = (i as i32 + k) as usize;
                if numbers[i][j] >= numbers[index][j] && k != 0 {
                    is_min = false;
                }
            }
            for k in startj..endj + 1 {
                let index = (j as i32 + k) as usize;
                if numbers[i][j] >= numbers[i][index] && k != 0 {
                    is_min = false;
                }
            }

            if is_min {
                mins.push(numbers[i][j])
            }
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
