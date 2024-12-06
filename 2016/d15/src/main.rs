fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut discs: Vec<Disc> = input
        .lines()
        .enumerate()
        .map(|(i, l)| Disc::from(l, i + 1))
        .collect();

    let mut p1 = 0;

    while !discs.iter().all(|d| d.is_open()) {
        discs.iter_mut().for_each(|d| d.tick());
        p1 += 1;
    }

    let mut p2 = p1;
    let cycle = discs.iter().map(|d| d.n).product::<usize>();

    loop {
        p2 += cycle;
        if (p2 + 7) % 11 == 0 {
            break;
        }
    }

    (p1, p2)
}

#[derive(Debug, Clone)]
struct Disc {
    position: usize,
    n: usize,
}

impl Disc {
    fn new(position: usize, n: usize) -> Self {
        Self { position, n }
    }

    fn from(line: &str, i: usize) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let n: usize = parts[3].parse().unwrap();
        let position: usize = parts[11].trim_end_matches('.').parse().unwrap();
        let position = (position + i) % n;
        Self::new(position, n)
    }

    fn tick(&mut self) {
        self.position = (self.position + 1) % self.n;
    }

    fn is_open(&self) -> bool {
        self.position == 0
    }
}
