fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(file: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(file).unwrap();
    let ips: Vec<Ipv7> = input.lines().map(Ipv7::from_line).collect();

    let mut c1 = 0;
    let mut c2 = 0;

    for ip in &ips {
        if ip.supports_tls() {
            c1 += 1;
        }
        if ip.supports_ssl() {
            c2 += 1;
        }
    }

    (c1, c2)
}

#[derive(Debug)]
struct Ipv7 {
    supernet: Vec<String>,
    hypernet: Vec<String>,
}

impl Ipv7 {
    fn from_line(line: &str) -> Self {
        let mut supernet = vec![];
        let mut hypernet = vec![];

        let parts = line
            .replace(['[', ']'], ",")
            .split(',')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        for (i, part) in parts.iter().enumerate() {
            if i % 2 == 0 {
                supernet.push(part.to_string());
            } else {
                hypernet.push(part.to_string());
            }
        }

        Self { supernet, hypernet }
    }

    fn has_abba(value: &str) -> bool {
        let mut abba = false;
        let chars: Vec<char> = value.chars().collect();
        for i in 3..chars.len() {
            if chars[i - 3] == chars[i] && chars[i - 2] == chars[i - 1] && chars[i - 1] != chars[i]
            {
                abba = true;
                break;
            }
        }
        abba
    }

    fn supports_tls(&self) -> bool {
        let mut support = false;

        for part in &self.supernet {
            if Self::has_abba(part) {
                support = true;
                break;
            }
        }

        if support {
            for part in &self.hypernet {
                if Self::has_abba(part) {
                    support = false;
                    break;
                }
            }
        }

        support
    }

    fn supports_ssl(&self) -> bool {
        for part in &self.supernet {
            let chars: Vec<char> = part.chars().collect();
            for i in 2..chars.len() {
                if chars[i - 2] == chars[i] && chars[i] != chars[i - 1] {
                    let aba = [chars[i - 1], chars[i], chars[i - 1]];
                    let aba = aba.iter().collect::<String>();
                    if self.hypernet_contains(&aba) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn hypernet_contains(&self, bab: &str) -> bool {
        for part in &self.hypernet {
            if part.contains(bab) {
                return true;
            }
        }
        false
    }
}
