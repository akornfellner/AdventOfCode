use std::fs;

pub fn solve(filename: &str, preamble: usize) -> usize {
    let input = fs::read_to_string(filename).unwrap();
    let numbers: Vec<usize> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();

    let mut result = 0usize;

    for i in preamble.. {
        if !has_sum(numbers[i], &numbers[i - preamble..i]) {
            result = numbers[i];
            break;
        }
    }

    let (min, max) = get_list(result, &numbers);

    min + max
}

fn get_list(number: usize, numbers: &[usize]) -> (usize, usize) {
    let mut start = 0usize;

    let mut sum = 0usize;

    let mut min = std::usize::MAX;
    let mut max = 0usize;

    loop {
        for i in start.. {
            sum += numbers[i];

            if numbers[i] < min {
                min = numbers[i];
            }
            if numbers[i] > max {
                max = numbers[i];
            }
            if sum == number {
                return (min, max);
            } else if sum > number {
                start += 1;
                sum = 0;
                min = std::usize::MAX;
                max = 0usize;
                break;
            }
        }
    }
}

fn has_sum(number: usize, numbers: &[usize]) -> bool {
    for a in numbers {
        for b in numbers {
            if a != b && a + b == number {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt", 5), 62);
    }
}
