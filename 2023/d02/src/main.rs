use std::fs;

fn main() {
    println!("Part One: {}", solve_one("input.in"));
    println!("Part Two: {}", solve_two("input.in"));
}

fn solve_one(input: &str) -> i32 {
    let input = fs::read_to_string(input).unwrap().replace(';', ",");
    let lines: Vec<&str> = input.lines().collect();

    let colors = ["red", "green", "blue"];
    let maxs = [12, 13, 14];

    let mut sum = 0;

    'outer: for line in lines {
        let parts = line.split(": ").collect::<Vec<&str>>();
        let game = parts[0].split_whitespace().collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();

        let cubes = parts[1].split(", ").collect::<Vec<&str>>();

        for cube in cubes {
            for (c, color) in colors.iter().enumerate() {
                if cube.contains(color) {
                    let value = cube.split_whitespace().collect::<Vec<&str>>();
                    let value = value[0].parse::<i32>().unwrap();

                    if value > maxs[c] {
                        continue 'outer;
                    }
                }
            }
        }

        sum += game;
    }

    sum
}

fn solve_two(input: &str) -> i32 {
    let input = fs::read_to_string(input).unwrap().replace(';', ",");
    let lines: Vec<&str> = input.lines().collect();

    let colors = ["red", "green", "blue"];

    let mut sum = 0;

    for line in lines {
        let parts = line.split(": ").collect::<Vec<&str>>();
        let cubes = parts[1].split(", ").collect::<Vec<&str>>();
        let mut mins = [0; 3];

        for cube in cubes {
            for (c, color) in colors.iter().enumerate() {
                if cube.contains(color) {
                    let value = cube.split_whitespace().collect::<Vec<&str>>();
                    let value = value[0].parse::<i32>().unwrap();

                    if value > mins[c] {
                        mins[c] = value;
                    }
                }
            }
        }

        sum += mins.iter().product::<i32>();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(solve_one("input_test.in"), 8);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_two("input_test.in"), 2286);
    }
}
