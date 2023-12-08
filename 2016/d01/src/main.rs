fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (i32, i32) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    let cmds = input.split(", ").map(Cmd::from).collect::<Vec<Cmd>>();
    let mut curr = 0;
    let mut pos = (0, 0);
    let mut visited: Vec<(i32, i32)> = vec![];

    for cmd in &cmds {
        curr = match cmd.direction {
            Direction::Right => (curr + 1) % 4,
            Direction::Left => (curr + 3) % 4,
        };

        match curr {
            0 => pos.0 -= cmd.value,
            1 => pos.1 -= cmd.value,
            2 => pos.0 += cmd.value,
            3 => pos.1 += cmd.value,
            _ => panic!("Invalid direction"),
        }
    }

    result.0 = pos.0.abs() + pos.1.abs();

    pos = (0, 0);
    curr = 0;

    'outer: for cmd in &cmds {
        curr = match cmd.direction {
            Direction::Right => (curr + 1) % 4,
            Direction::Left => (curr + 3) % 4,
        };
        for _ in 0..cmd.value {
            match curr {
                0 => pos.0 -= 1,
                1 => pos.1 -= 1,
                2 => pos.0 += 1,
                3 => pos.1 += 1,
                _ => panic!("Invalid direction"),
            }

            if visited.contains(&pos) {
                result.1 = pos.0.abs() + pos.1.abs();
                break 'outer;
            } else {
                visited.push(pos);
            }
        }
    }

    result
}

struct Cmd {
    direction: Direction,
    value: i32,
}

impl Cmd {
    fn from(input: &str) -> Self {
        let parts = input.chars().collect::<Vec<char>>();
        let direction = match parts[0] {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Invalid direction"),
        };
        let value = parts[1..]
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        Cmd { direction, value }
    }
}

enum Direction {
    Right,
    Left,
}
