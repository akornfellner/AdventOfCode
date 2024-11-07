use std::cmp::Ordering;

fn main() {
    println!("Part one: {}", solve("qzyelonm", false));
    println!("Part one: {}", solve("abc", true));
}

fn solve(salt: &str, two: bool) -> usize {
    let mut tries: Vec<(Hash, char)> = vec![];
    let mut valid: Vec<Hash> = vec![];
    let mut count = 0usize;
    let mut index = 0usize;

    loop {
        let hash = Hash::compute(salt, index, two);

        if count >= 64 && hash.index > valid[63].index + 1000 {
            break;
        }

        for (h, c) in &tries {
            if h.index + 1000 >= hash.index && hash.contains_five(*c) {
                valid.push(h.clone());
                count += 1;
            }
        }

        if let Some(c) = hash.contains_three() {
            tries.push((hash.clone(), c));
        }

        index += 1;
    }

    valid.sort();

    for (i, v) in valid.iter().enumerate() {
        println!("{}: {}", i + 1, v.index);
    }

    valid[63].index
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Hash {
    hash: String,
    index: usize,
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

            Hash { hash, index }
        } else {
            Hash { hash, index }
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

impl Ord for Hash {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialOrd for Hash {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
