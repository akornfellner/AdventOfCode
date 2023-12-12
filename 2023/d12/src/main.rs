use std::collections::{HashMap, HashSet};
use std::fmt;

fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    let rows: Vec<Row> = input.lines().map(|r| Row::from(r, false)).collect();
    let mut calculated: Calc = HashMap::new();

    for (i, row) in rows.iter().enumerate() {
        let mut possibilities: HashSet<Row> = HashSet::new();
        possibilities.insert(row.clone());
        row.replace_all(&mut possibilities, &mut calculated);
        for p in possibilities {
            if p.is_valid() {
                result.0 += 1;
            }
        }
        println!("{}/{}", i + 1, rows.len());
    }
    result
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Row {
    groups: Vec<(usize, char)>,
    numbers: Vec<usize>,
}

impl Row {
    fn from(line: &str, two: bool) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let chars: Vec<char> = parts[0].chars().collect();

        let mut count = 1;
        let mut prev = chars[0];
        let mut groups = vec![];

        for c in chars.iter().skip(1) {
            if *c == prev {
                count += 1;
            } else {
                groups.push((count, prev));
                count = 1;
                prev = *c;
            }
        }

        groups.push((count, prev));

        let numbers: Vec<usize> = parts[1]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        if two {
            let mut new_groups = vec![];
            let mut new_numbers = vec![];

            for i in 0..5 {
                for g in &groups {
                    new_groups.push(*g);
                }
                if i < 4 {
                    new_groups.push((1, '?'));
                }
                for n in &numbers {
                    new_numbers.push(*n);
                }
            }

            let groups = new_groups;
            let numbers = new_numbers;
        }

        Self { groups, numbers }
    }

    fn is_valid(&self) -> bool {
        let mut checks: Vec<usize> = vec![];

        for (n, c) in &self.groups {
            if *c == '#' {
                checks.push(*n);
            }
        }
        checks == self.numbers
    }

    fn combine(&mut self) {
        let mut groups_new: Vec<(usize, char)> = vec![];
        let mut count = self.groups[0].0;
        let mut prev = self.groups[0].1;

        for (n, c) in self.groups.iter().skip(1) {
            if *c == prev {
                count += n;
            } else {
                groups_new.push((count, prev));
                count = *n;
                prev = *c;
            }
        }

        groups_new.push((count, prev));
        self.groups = groups_new;
    }

    fn replace(n: usize, possibilities: &mut HashSet<Self>) {
        if n == 1 && possibilities.len() == 0 {
            possibilities.insert(Self {
                groups: vec![(1, '#')],
                numbers: vec![],
            });
            possibilities.insert(Self {
                groups: vec![(1, '.')],
                numbers: vec![],
            });
            return;
        }
        if n <= 1 {
            return;
        }

        if possibilities.len() == 0 {
            possibilities.insert(Self {
                groups: vec![(1, '#')],
                numbers: vec![],
            });
            possibilities.insert(Self {
                groups: vec![(1, '.')],
                numbers: vec![],
            });
        }

        let mut new_possibilities: HashSet<Row> = HashSet::new();

        for p in possibilities.iter() {
            for i in 0..=p.groups.len() {
                let mut new_p1 = p.clone();
                let mut new_p2 = p.clone();
                new_p1.groups.insert(i, (1, '#'));
                new_p2.groups.insert(i, (1, '.'));
                new_p1.combine();
                new_p2.combine();
                new_possibilities.insert(new_p1);
                new_possibilities.insert(new_p2);
            }
        }
        *possibilities = new_possibilities;

        Self::replace(n - 1, possibilities);
    }

    fn replace_all(&self, possibilities: &mut HashSet<Self>, calculated: &mut Calc) {
        let mut new_possibilities: HashSet<Self> = HashSet::new();
        let mut found = false;

        for p in possibilities.iter() {
            let mut i = 0;
            let mut n = 0;
            for (j, (k, c)) in p.groups.iter().enumerate() {
                if *c == '?' {
                    i = j;
                    n = *k;
                    found = true;
                    break;
                }
            }

            if found {
                let mut values: HashSet<Self> = HashSet::new();
                if calculated.contains_key(&n) {
                    values = calculated.get(&n).unwrap().clone();
                } else {
                    Self::replace(n, &mut values);
                    calculated.insert(n, values.clone());
                }

                for v in &values {
                    let mut new_p: Vec<(usize, char)> = vec![];
                    let before = p.groups[..i].to_vec();
                    let after = p.groups[i + 1..].to_vec();

                    new_p.extend(before.iter());
                    new_p.extend(v.groups.iter());
                    new_p.extend(after.iter());

                    let mut row = Row {
                        groups: new_p,
                        numbers: self.numbers.clone(),
                    };
                    row.combine();

                    new_possibilities.insert(row);
                }
            }
        }
        if !found {
            return;
        }

        *possibilities = new_possibilities;

        self.replace_all(possibilities, calculated);
    }
}

type Calc = HashMap<usize, HashSet<Row>>;

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (n, c) in &self.groups {
            for _ in 0..*n {
                write!(f, "{}", c)?;
            }
        }
        write!(f, " {:?}", self.numbers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve("input_test.in").0, 21);
    }

    #[test]
    fn test_two() {
        assert_eq!(solve("input_test.in").1, 0);
    }
}
