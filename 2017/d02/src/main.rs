use std::fs;

fn main() {
    println!("Part One: {}", solve("input"));
    println!("Part Two: {}", solve_two("input"));
}

fn solve(input: &str) -> usize {
    let input: String = fs::read_to_string(input).unwrap();
    let mut numbers: Vec<Vec<usize>> = vec![];

    for line in input.lines() {
        let mut line_numbers: Vec<usize> = vec![];
        for number in line.split('\t') {
            line_numbers.push(number.parse::<usize>().unwrap());
        }
        numbers.push(line_numbers);
    }

    let mut sum = 0;

    for line in numbers {
        let mut min = line[0];
        let mut max = line[0];

        for number in line {
            if number < min {
                min = number;
            }
            if number > max {
                max = number;
            }
        }

        sum += max - min;
    }

    sum
}

fn solve_two(input: &str) -> usize {
    let input: String = fs::read_to_string(input).unwrap();
    let mut numbers: Vec<Vec<usize>> = vec![];

    for line in input.lines() {
        let mut line_numbers: Vec<usize> = vec![];
        for number in line.split('\t') {
            line_numbers.push(number.parse::<usize>().unwrap());
        }
        numbers.push(line_numbers);
    }

    let mut sum = 0;

    for line in numbers {
        for (i, number) in line.iter().enumerate() {
            for (j, number2) in line.iter().enumerate() {
                if i != j && number % number2 == 0 {
                    sum += number / number2;
                }
            }
        }
    }

    sum
}
