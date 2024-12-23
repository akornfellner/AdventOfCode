fn main() {
    println!("Part one: {}", solve("qzyelonm", false));
    println!("Part one: {}", solve("qzyelonm", true));
}

fn solve(salt: &str, two: bool) -> usize {
    let mut tries: Vec<(usize, char)> = vec![];
    let mut valid: Vec<usize> = vec![];
    let mut count = 0usize;
    let mut index = 0usize;

    loop {
        let hash = Hash::compute(salt, index, two);

        if count >= 64 && index > valid[63] + 1000 {
            break;
        }

        for (i, c) in &tries {
            if i + 1000 >= index && !valid.contains(i) && hash.contains_five(*c) {
                valid.push(*i);
                count += 1;
            }
        }

        if let Some(c) = hash.contains_three() {
            tries.push((index, c));
        }

        index += 1;
    }

    valid.sort();

    valid[63]
}

#[derive(Debug, Clone)]
struct Hash {
    hash: String,
}

impl Hash {
    fn compute(salt: &str, index: usize, two: bool) -> Self {
        let s = format!("{}{}", salt, index);
        let hash = format!("{:x}", md5::compute(s.as_bytes()));

        if two {
            let mut hash = hash.clone();
            for _ in 0..2016 {
                hash = format!("{:x}", md5::compute(hash.as_bytes()));
            }

            Hash { hash }
        } else {
            Hash { hash }
        }
    }

    fn contains_three(&self) -> Option<char> {
        let mut chars = self.hash.chars();
        let mut c = chars.next().unwrap();
        let mut count = 1;

        for ch in chars {
            if ch == c {
                count += 1;
                if count == 3 {
                    return Some(c);
                }
            } else {
                c = ch;
                count = 1;
            }
        }

        None
    }

    fn contains_five(&self, c: char) -> bool {
        let sub = String::from(c).repeat(5);
        self.hash.contains(&sub)
    }
}
