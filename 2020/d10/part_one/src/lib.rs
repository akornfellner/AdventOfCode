use std::fs;

pub fn solve(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).unwrap();

    let mut numbers: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    numbers.push(0);

    numbers.sort_unstable();

    let mut count_1 = 0;
    let mut count_3 = 1;

    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];
        match diff {
            1 => count_1 += 1,
            3 => count_3 += 1,
            _ => {}
        }
    }

    count_1 * count_3
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt"), 35);
    }
}
