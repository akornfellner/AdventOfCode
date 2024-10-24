use std::{collections::HashMap, fs};

fn main() {
    let result = solve("input.txt");
    println!("Part one: {}", result.0);
    println!("Part two: {}", result.1);
}

fn solve(file: &str) -> (usize, usize) {
    let input = fs::read_to_string(file).unwrap();
    let rooms: Vec<Room> = input.lines().map(Room::from_line).collect();

    let number = rooms
        .iter()
        .filter(|room| room.is_valid())
        .map(|room| room.id)
        .sum();

    let northpole_id = rooms
        .iter()
        .find(|room| room.get_real_name().contains("northpole"))
        .unwrap()
        .id;

    (number, northpole_id)
}

#[derive(Debug)]
struct Room {
    name: String,
    id: usize,
    checksum: String,
}

impl Room {
    fn from_line(line: &str) -> Self {
        let parts: Vec<&str> = line.split('[').collect();
        let checksum = parts[1].trim_end_matches(']');
        let parts: Vec<&str> = parts[0].split('-').collect();
        let id = parts[parts.len() - 1].parse::<usize>().unwrap();
        let name = parts[..parts.len() - 1].join(" ");

        Room {
            name,
            id,
            checksum: checksum.to_string(),
        }
    }

    fn get_real_name(&self) -> String {
        let mut real_name = String::new();

        for c in self.name.chars() {
            if c.is_alphabetic() {
                let new_char = ((((c as u8 - b'a') as usize + self.id) % 26) as u8 + b'a') as char;
                real_name.push(new_char);
            } else {
                real_name.push(c);
            }
        }

        real_name
    }

    fn is_valid(&self) -> bool {
        let mut map: HashMap<char, usize> = HashMap::new();

        for c in self.name.chars() {
            if c.is_alphabetic() {
                let count = map.entry(c).or_insert(0);
                *count += 1;
            }
        }

        let mut pairs: Vec<Pair> = map
            .iter()
            .map(|(key, value)| Pair {
                key: *key,
                value: *value,
            })
            .collect();

        pairs.sort();

        let mut checksum = String::new();

        for pair in pairs.iter().take(5) {
            checksum.push(pair.key);
        }

        checksum == self.checksum
    }
}

struct Pair {
    key: char,
    value: usize,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .value
            .cmp(&self.value)
            .then_with(|| self.key.cmp(&other.key))
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.key == other.key
    }
}

impl Eq for Pair {}
