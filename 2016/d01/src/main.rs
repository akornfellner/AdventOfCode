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

    for cmd in cmds {
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

    println!("{:?}", pos);

    result.0 = pos.0.abs() + pos.1.abs();

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
        let value = parts[1].to_digit(10).unwrap() as i32;

        Cmd { direction, value }
    }
}

enum Direction {
    Right,
    Left,
}
