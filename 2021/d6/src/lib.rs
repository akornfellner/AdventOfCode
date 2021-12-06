use std::fs;

pub fn solve(filename: &str, rounds: i32) -> usize {
    let input = fs::read_to_string(filename).unwrap();

    let numbers: Vec<usize> = input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut fishes = [0usize; 9];

    for i in 0..fishes.len() {
        for n in &numbers {
            if &i == n {
                fishes[i] += 1;
            }
        }
    }

    for _ in 0..rounds {
        round(&mut fishes);
    }

    let sum: usize = fishes.into_iter().sum();

    sum
}

fn round(fishes: &mut [usize]) {
    let zeros = fishes[0];

    for i in 1..fishes.len() {
        fishes[i - 1] = fishes[i];
    }

    fishes[6] += zeros;
    fishes[8] = zeros;
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve18() {
        assert_eq!(solve("input_test.txt", 18), 26);
    }

    #[test]
    fn test_solve80() {
        assert_eq!(solve("input_test.txt", 80), 5934);
    }

    #[test]
    fn test_solve256() {
        assert_eq!(solve("input_test.txt", 256), 26984457539);
    }
}
