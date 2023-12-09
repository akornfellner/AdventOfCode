fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (String, String) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (String::new(), String::new());

    let board1 = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];
    let board2 = vec![
        vec!['-', '-', '1', '-', '-'],
        vec!['-', '2', '3', '4', '-'],
        vec!['5', '6', '7', '8', '9'],
        vec!['-', 'A', 'B', 'C', '-'],
        vec!['-', '-', 'D', '-', '-'],
    ];

    let mut pos1 = (1, 1);
    let mut pos2 = (2, 0);

    for cmd in input.lines() {
        for c in cmd.chars() {
            let mut new = pos2;
            match c {
                'L' => {
                    pos1.1 -= 1;
                    new.1 -= 1;
                }
                'R' => {
                    pos1.1 += 1;
                    new.1 += 1;
                }
                'U' => {
                    pos1.0 -= 1;
                    new.0 -= 1;
                }
                'D' => {
                    pos1.0 += 1;
                    new.0 += 1;
                }
                _ => (),
            }
            correct(&mut pos1, &board1);
            correct(&mut new, &board2);
            if !(board2[new.0 as usize][new.1 as usize] == '-') {
                pos2 = new;
            }
        }
        result.0 += &board1[pos1.0 as usize][pos1.1 as usize].to_string();
        result.1 += &board2[pos2.0 as usize][pos2.1 as usize].to_string();
    }

    result
}

fn correct(pos: &mut (i32, i32), board: &[Vec<char>]) {
    if pos.0 < 0 {
        pos.0 = 0
    };
    if pos.0 >= board.len() as i32 {
        pos.0 = (board.len() - 1) as i32
    };
    if pos.1 < 0 {
        pos.1 = 0
    };
    if pos.1 >= board[pos.0 as usize].len() as i32 {
        pos.1 = (board[pos.0 as usize].len() - 1) as i32
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve("input_test.in").0, String::from("1985"));
    }

    #[test]
    fn test_two() {
        assert_eq!(solve("input_test.in").1, String::from("5DB3"));
    }
}
