fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    let start: Field = input.lines().map(Rock::from).collect();
    result.0 = total_load(&start, false);

    let mut field = start.clone();

    let mut fields: Vec<Field> = vec![start.clone()];

    let mut end: usize = 0;

    for i in 1..1000000000 {
        cycle(&mut field);
        if fields.contains(&field) {
            end = i;
            break;
        }
        fields.push(field.clone());
    }

    let start = fields.iter().position(|x| x == &field).unwrap();
    let length = end - start;

    let rest = (1000000000 - start) % length;

    for _ in 0..rest {
        cycle(&mut field);
    }

    result.1 = total_load(&field, true);

    result
}

fn total_load(field: &[Vec<Rock>], two: bool) -> usize {
    let mut result = 0;
    for i in 0..field[0].len() {
        let mut m: usize = field.len();
        for j in 0..field.len() {
            if two {
                match field[j][i] {
                    Rock::Round => {
                        result += m;
                        m -= 1;
                    }
                    _ => m -= 1,
                }
            } else {
                match field[j][i] {
                    Rock::Cube => m = field.len() - j - 1,
                    Rock::Round => {
                        result += m;
                        m -= 1;
                    }
                    Rock::Empty => (),
                }
            }
        }
    }
    result
}

fn cycle(field: &mut [Vec<Rock>]) {
    for j in 0..field[0].len() {
        let mut start: usize = 0;
        for i in 0..field.len() {
            match field[i][j] {
                Rock::Cube => start = i + 1,
                Rock::Round => {
                    field[i][j] = Rock::Empty;
                    field[start][j] = Rock::Round;
                    start += 1;
                }

                Rock::Empty => (),
            }
        }
    }

    for i in 0..field.len() {
        let mut start = 0;
        for j in 0..field[0].len() {
            match field[i][j] {
                Rock::Cube => start = j + 1,
                Rock::Round => {
                    field[i][j] = Rock::Empty;
                    field[i][start] = Rock::Round;
                    start += 1;
                }
                Rock::Empty => (),
            }
        }
    }

    for j in 0..field[0].len() {
        let mut start: usize = field.len() - 1;
        for i in 0..field.len() {
            let i = field.len() - 1 - i;
            match field[i][j] {
                Rock::Cube => {
                    if i > 0 {
                        start = i - 1
                    }
                }
                Rock::Round => {
                    field[i][j] = Rock::Empty;
                    field[start][j] = Rock::Round;
                    if i > 0 {
                        start -= 1;
                    }
                }
                Rock::Empty => (),
            }
        }
    }

    for i in 0..field.len() {
        let mut start = field[0].len() - 1;
        for j in 0..field[0].len() {
            let j = field[0].len() - 1 - j;
            match field[i][j] {
                Rock::Cube => {
                    if j > 0 {
                        start = j - 1
                    }
                }
                Rock::Round => {
                    field[i][j] = Rock::Empty;
                    field[i][start] = Rock::Round;
                    if j > 0 {
                        start -= 1;
                    }
                }
                Rock::Empty => (),
            }
        }
    }
}

type Field = Vec<Vec<Rock>>;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Rock {
    Cube,
    Round,
    Empty,
}

impl Rock {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Rock::Cube,
            'O' => Rock::Round,
            _ => Rock::Empty,
        }
    }

    fn from(line: &str) -> Vec<Self> {
        line.chars().map(Rock::from_char).collect()
    }
}
