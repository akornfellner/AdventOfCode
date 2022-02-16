mod scanner;

use std::{collections::HashMap, fs};

use scanner::{Beacon, Scanner};

pub fn solve_one(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();

    let input: Vec<&str> = input.split("\n\n").collect();

    let mut scanners: Vec<Scanner> = vec![];

    let mut id = 0usize;

    for line in input {
        scanners.push(Scanner::new(id, line));
        id += 1;
    }

    let mut combine: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for i in 0..scanners.len() {
        for j in 0..scanners.len() {
            if i < j {
                let a = scanners[i].clone();
                let b = scanners[j].clone();

                let mut count = 0usize;
                let mut pairs: HashMap<usize, Vec<usize>> = HashMap::new();

                for ((b1, b2), dist1) in &a.dists {
                    for ((b3, b4), dist2) in &b.dists {
                        if dist1 == dist2 {
                            let b = *b1;
                            if pairs.contains_key(&b) {
                                if pairs[&b].len() != 1 {
                                    let p = pairs[&b].clone();
                                    if p.contains(&b3) {
                                        pairs.insert(b, vec![*b3]);
                                    }
                                    if p.contains(&b4) {
                                        pairs.insert(b, vec![*b4]);
                                    }
                                }
                            } else {
                                let partners = vec![*b3, *b4];
                                pairs.insert(b, partners);
                            }

                            let b = *b2;
                            if pairs.contains_key(&b) {
                                if pairs[&b].len() != 1 {
                                    let p = pairs[&b].clone();
                                    if p.contains(&b3) {
                                        pairs.insert(b, vec![*b3]);
                                    }
                                    if p.contains(&b4) {
                                        pairs.insert(b, vec![*b4]);
                                    }
                                }
                            } else {
                                let partners = vec![*b3, *b4];
                                pairs.insert(b, partners);
                            }

                            count += 1;
                        }
                    }
                }

                if count >= 66 {
                    let mut new: Vec<(usize, usize)> = vec![];
                    for (k, v) in pairs {
                        new.push((k, v[0]));
                    }
                    combine.insert((scanners[i].id, scanners[j].id), new);
                }
            }
        }
    }

    let test = combine[&(0, 1)];
    let (a1,b1)=

    5
}

#[cfg(test)]
mod tests {
    use crate::solve_one;

    #[test]
    fn one_works() {
        assert_eq!(solve_one("input_test.txt"), 79);
    }
}
