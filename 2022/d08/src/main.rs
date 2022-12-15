use std::fs;

fn main() {
    println!("part one: {}", solve_one("input.txt"));
    println!("part two: {}", solve_two("input.txt"));
}

fn solve_one(input: &str) -> i32 {
    let mut forest: Vec<Vec<(i32, bool)>> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| (y as i32 - '0' as i32, false))
                .collect::<Vec<(i32, bool)>>()
        })
        .collect();

    for row in &mut forest {
        let mut current = -1;
        for entry in row {
            if entry.0 > current {
                current = entry.0;
                entry.1 = true;
            }
        }
    }

    for row in &mut forest {
        let mut current = -1;
        for j in (0..row.len()).rev() {
            let entry = &mut row[j];
            if entry.0 > current {
                current = entry.0;
                entry.1 = true;
            }
        }
    }

    for j in 0..forest[0].len() {
        let mut current = -1;
        for row in &mut forest {
            let entry = &mut row[j];
            if entry.0 > current {
                current = entry.0;
                entry.1 = true;
            }
        }
    }

    for j in 0..forest[0].len() {
        let mut current = -1;
        for i in (0..forest.len()).rev() {
            let entry = &mut forest[i][j];
            if entry.0 > current {
                current = entry.0;
                entry.1 = true;
            }
        }
    }

    let mut result = 0;

    for row in &forest {
        for entry in row {
            result += i32::from(entry.1);
        }
    }

    result
}

fn solve_two(input: &str) -> i32 {
    let forest: Vec<Vec<i32>> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| y as i32 - '0' as i32)
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut result = -1;

    for (i, row) in forest.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            let mut mult = 1;
            let mut count = 0;

            for x in (0..i).rev() {
                count += 1;

                if forest[x][j] >= *tree {
                    break;
                }
            }

            mult *= count;
            count = 0;

            for item in forest.iter().skip(i + 1) {
                count += 1;

                if item[j] >= *tree {
                    break;
                }
            }

            mult *= count;
            count = 0;

            for x in (0..j).rev() {
                count += 1;

                if forest[i][x] >= *tree {
                    break;
                }
            }

            mult *= count;
            count = 0;

            for x in j + 1..row.len() {
                count += 1;

                if forest[i][x] >= *tree {
                    break;
                }
            }

            mult *= count;

            if mult > result {
                result = mult;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve_one("input_test.txt");
        assert_eq!(result, 21);
    }

    #[test]
    fn two_works() {
        let result = solve_two("input_test.txt");
        assert_eq!(result, 8);
    }
}
