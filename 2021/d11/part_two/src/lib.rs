use std::fs;

pub fn solve(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let mut numbers: Vec<Vec<(i32, bool)>> = vec![];

    for line in lines {
        let mut l: Vec<(i32, bool)> = vec![];
        for c in line.chars() {
            l.push((c as i32 - 48, false));
        }
        numbers.push(l);
    }

    let mut r = 0;

    loop {
        r += 1;
        let s = round(&mut numbers);
        if s {
            break;
        }
    }

    r
}

fn round(numbers: &mut Vec<Vec<(i32, bool)>>) -> bool {
    let rows = numbers.len();
    let cols = numbers[0].len();

    for i in 0..rows {
        for j in 0..cols {
            flash(i, j, numbers);
        }
    }

    let mut flashes = 0;

    for line in numbers {
        for number in line {
            if number.1 {
                number.0 = 0;
                number.1 = false;
                flashes += 1;
            }
        }
    }

    if flashes == rows * cols {
        return true;
    }

    false
}

fn flash(i: usize, j: usize, numbers: &mut Vec<Vec<(i32, bool)>>) {
    let number = &mut numbers[i][j];
    number.0 += 1;

    if !number.1 && number.0 > 9 {
        number.1 = true;

        let rows = numbers.len();
        let cols = numbers[0].len();

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

        for m in starti..endi + 1 {
            for n in startj..endj + 1 {
                if !(m == 0 && n == 0) {
                    let newi = (i as i32 + m) as usize;
                    let newj = (j as i32 + n) as usize;
                    flash(newi, newj, numbers);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt"), 195);
    }
}
