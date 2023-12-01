use std::fs;

fn main() {
    let (part_one, part_two) = solve("input.in");
    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

fn solve(input: &str) -> (usize, usize) {
    let input = fs::read_to_string(input).unwrap();
    let mut numbers: Vec<i32> = input.split('\t').map(|x| x.parse().unwrap()).collect();
    let n = numbers.len();
    let mut seen: Vec<Vec<i32>> = Vec::new();
    seen.push(numbers.clone());

    let mut count = 0;
    let cycle;

    loop {
        let max_index = get_max_index(&numbers);
        let mut max = numbers[max_index];
        numbers[max_index] = 0;

        let add = max / n as i32;
        let mut index = max_index;

        for number in &mut numbers {
            *number += add;
        }

        max -= add * n as i32;

        while max > 0 {
            index = (index + 1) % n;
            numbers[index] += 1;
            max -= 1;
        }

        count += 1;

        if seen.contains(&numbers) {
            cycle = count - seen.iter().position(|x| x == &numbers).unwrap();
            break;
        }

        seen.push(numbers.clone());
    }

    (count, cycle)
}

fn get_max_index(numbers: &[i32]) -> usize {
    let mut max = numbers[0];
    let mut max_index = 0;
    for (i, &number) in numbers.iter().enumerate() {
        if number > max {
            max = number;
            max_index = i;
        }
    }
    max_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        assert_eq!(solve("input_test.in").0, 5);
    }

    #[test]
    fn test_cycle() {
        assert_eq!(solve("input_test.in").1, 4);
    }
}
