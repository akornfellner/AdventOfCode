use stopwatch::time;

#[time]
fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();

    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let mut field: Vec<Vec<Pos>> = vec![];
    let mut robot = (0, 0);

    for (y, line) in parts[0].lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            let pos = Pos::from_char(c);
            if let Pos::Robot = pos {
                robot = (x, y);
            }
            row.push(pos);
        }
        field.push(row);
    }

    let moves = parts[1].replace("\n", "");
    let moves = moves.chars().map(Dir::from_char).collect::<Vec<Dir>>();

    for m in &moves {
        let (moved, new_pos) = make_move(robot, *m, &mut field);
        if moved {
            robot = new_pos;
        }
    }

    (check_sum(&field), 0)
}

fn make_move(pos: (usize, usize), dir: Dir, field: &mut Vec<Vec<Pos>>) -> (bool, (usize, usize)) {
    let (x, y) = pos;
    let t = field[y][x];
    let (dx, dy) = match dir {
        Dir::Up => (0, -1),
        Dir::Down => (0, 1),
        Dir::Left => (-1, 0),
        Dir::Right => (1, 0),
    };

    let new_x = (x as i32 + dx) as usize;
    let new_y = (y as i32 + dy) as usize;

    match field[new_y][new_x] {
        Pos::Wall => (false, pos),
        Pos::Box => {
            let (pushed, _) = make_move((new_x, new_y), dir, field);
            if pushed {
                field[y][x] = Pos::Empty;
                field[new_y][new_x] = t;
                (true, (new_x, new_y))
            } else {
                (false, pos)
            }
        }
        _ => {
            field[y][x] = Pos::Empty;
            field[new_y][new_x] = t;
            (true, (new_x, new_y))
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Pos {
    Robot,
    Wall,
    Empty,
    Box,
}

impl Pos {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Pos::Wall,
            '.' => Pos::Empty,
            '@' => Pos::Robot,
            'O' => Pos::Box,
            _ => panic!("Unknown character: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Dir::Up,
            'v' => Dir::Down,
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => panic!("Unknown direction: {}", c),
        }
    }
}

fn pring_field(field: &[Vec<Pos>]) {
    for row in field {
        for pos in row {
            match pos {
                Pos::Robot => print!("@"),
                Pos::Wall => print!("#"),
                Pos::Empty => print!("."),
                Pos::Box => print!("O"),
            }
        }
        println!();
    }
}

fn check_sum(field: &[Vec<Pos>]) -> usize {
    let mut result = 0;
    for (y, row) in field.iter().enumerate() {
        for (x, pos) in row.iter().enumerate() {
            if let Pos::Box = pos {
                result += y * 100 + x;
            }
        }
    }
    result
}
