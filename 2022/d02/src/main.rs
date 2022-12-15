use std::fs;

fn main() {
    println!("part one: {}", solve("input.txt", false));
    println!("part two: {}", solve("input.txt", true));
}

fn solve(input: &str, two: bool) -> i32 {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut result = 0;

    for line in lines {
        let parts: Vec<char> = line.chars().collect();
        result += get_result(parts[0], parts[2], two);
    }

    result
}

fn get_result(opp: char, me: char, two: bool) -> i32 {
    let opp = opp as i32 - 'A' as i32;
    let me = me as i32 - 'X' as i32;

    (if !two {
        let diff = (me - opp + 3) % 3;

        me + match diff {
            1 => 6,
            2 => 0,
            _ => 3,
        }
    } else {
        match me {
            0 => (opp + 2) % 3,
            1 => 3 + opp,
            _ => 6 + (opp + 1) % 3,
        }
    }) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve("input_test.txt", false);
        assert_eq!(result, 15);
    }

    #[test]
    fn two_works() {
        let result = solve("input_test.txt", true);
        assert_eq!(result, 12);
    }
}
