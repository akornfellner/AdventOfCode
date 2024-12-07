fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (u64, u64) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let equation = Equation::from(line);
        if equation.is_valid(0, 0, false) {
            p1 += equation.result;
        }
        if equation.is_valid(0, 0, true) {
            p2 += equation.result;
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
    let mut n = first;
    let mut m = second;
    while m > 0 {
        n *= 10;
        m /= 10;
    }
    n + second
}
