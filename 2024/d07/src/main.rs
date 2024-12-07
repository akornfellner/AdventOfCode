fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (u64, u64) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut p1 = 0;
    let mut p2 = 0;

    let equations = input.lines().map(Equation::from).collect::<Vec<Equation>>();

    for eq in &equations {
        if eq.is_valid(0, 0, false) {
            p1 += eq.result;
        }
        if eq.is_valid(0, 0, true) {
            p2 += eq.result;
        }
    }

    (p1, p2)
}

#[derive(Debug)]
struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn from(line: &str) -> Self {
        let parts = line.split(": ").collect::<Vec<&str>>();
        let result = parts[0].parse::<u64>().unwrap();
        let numbers: Vec<u64> = parts[1]
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        Self { result, numbers }
    }

    fn is_valid(&self, value: u64, depth: usize, two: bool) -> bool {
        if depth == self.numbers.len() {
            return self.result == value;
        }
        self.is_valid(value + self.numbers[depth], depth + 1, two)
            || self.is_valid(value * self.numbers[depth], depth + 1, two)
            || (two && self.is_valid(concat(value, self.numbers[depth]), depth + 1, two))
    }
}

fn concat(first: u64, second: u64) -> u64 {
    (first.to_string() + &second.to_string())
        .parse::<u64>()
        .unwrap()
}
