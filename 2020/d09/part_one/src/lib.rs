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

    result
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
        assert_eq!(solve("input_test.txt", 5), 127);
    }
}
