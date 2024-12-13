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

    let machines: Vec<Machine> = input.split("\n\n").map(Machine::from).collect();

    let mut result = [0, 0];

    for (i, p) in result.iter_mut().enumerate() {
        let add = if i == 0 { 0.0 } else { 10000000000000.0 };
        for machine in &machines {
            let (xa, ya) = (machine.a.0 as f64, machine.a.1 as f64);
            let (xb, yb) = (machine.b.0 as f64, machine.b.1 as f64);
            let (xp, yp) = (machine.prize.0 as f64 + add, machine.prize.1 as f64 + add);
            let a = (xp * yb - xb * yp) / (xa * yb - xb * ya);
            let b = (xa * yp - xp * ya) / (xa * yb - xb * ya);
            if a.fract() == 0.0 && b.fract() == 0.0 {
                *p += 3 * a as usize + b as usize;
            }
        }
    }

    (result[0], result[1])
}

#[derive(Debug)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

impl Machine {
    fn from(machine: &str) -> Self {
        let lines = machine.lines().collect::<Vec<&str>>();
        let a = lines[0];
        let b = lines[1];
        let prize = lines[2];

        let a = Machine::parse_line(a, true);
        let b = Machine::parse_line(b, true);
        let prize = Machine::parse_line(prize, false);

        Self { a, b, prize }
    }

    fn parse_line(line: &str, button: bool) -> (usize, usize) {
        let (i, sign) = if button { (2, '+') } else { (1, '=') };
        let parts = line.replace(',', "");
        let parts = parts.split_whitespace().collect::<Vec<&str>>();
        let x = parts[i].split(sign).collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();
        let y = parts[i + 1].split(sign).collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();

        (x, y)
    }
}
