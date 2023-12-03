use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.in").unwrap();
    let field: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut p1 = 0;
    let mut map: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for (x, line) in field.iter().enumerate() {
        let mut actual_number: Vec<char> = vec![];
        let mut found_symbol = false;
        let mut gears: HashSet<(usize, usize)> = HashSet::new();

        for (y, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                actual_number.push(*c);

                for (i, j) in get_neighbors(x, y, &field) {
                    if is_special_sign(field[i][j]) {
                        if field[i][j] == '*' {
                            gears.insert((i, j));
                        }
                        found_symbol = true;
                        break;
                    }
                }
            } else {
                if found_symbol {
                    let number = get_number(&actual_number);
                    p1 += number;
                    add_gears(number, &gears, &mut map);
                }
                found_symbol = false;
                actual_number = vec![];
                gears = HashSet::new();
            }
        }
        if found_symbol {
            let number = get_number(&actual_number);
            p1 += number;
            add_gears(number, &gears, &mut map);
        }
    }

    let mut p2 = 0;

    for (_, numbers) in map {
        if numbers.len() == 2 {
            p2 += numbers[0] * numbers[1];
        }
    }

    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn get_neighbors(x: usize, y: usize, arr: &[Vec<char>]) -> Vec<(usize, usize)> {
    let start_x = if x == 0 { 0 } else { x - 1 };
    let start_y = if y == 0 { 0 } else { y - 1 };
    let end_x = if x == arr.len() - 1 { x } else { x + 1 };
    let end_y = if y == arr[0].len() - 1 { y } else { y + 1 };

    let mut neighbors: Vec<(usize, usize)> = vec![];
    for i in start_x..=end_x {
        for j in start_y..=end_y {
            if i == 0 && j == 0 {
                continue;
            }
            neighbors.push((i, j));
        }
    }
    neighbors
}

fn is_special_sign(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

fn get_number(actual_number: &[char]) -> i32 {
    let mut number = String::new();
    for n in actual_number {
        number.push(*n);
    }
    number.parse::<i32>().unwrap()
}

fn add_gears(n: i32, gears: &HashSet<(usize, usize)>, map: &mut HashMap<(usize, usize), Vec<i32>>) {
    for gear in gears {
        let numbers = map.entry(*gear).or_default();
        numbers.push(n);
    }
}
