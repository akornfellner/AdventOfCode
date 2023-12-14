fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    let field: Vec<Vec<Rock>> = input.lines().map(Rock::from).collect();

    for i in 0..field[0].len() {
        let mut m = field.len();
        for j in 0..field.len() {
            match field[j][i] {
                Rock::Cube => m = field.len() - j - 1,
                Rock::Round => {
                    result.0 += m;
                    m -= 1;
                }
                Rock::Empty => (),
            }
        }
    }

    result
}

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
