use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Scanner {
    pub id: usize,
    pub beacons: HashMap<usize, Beacon>,
    pub dists: HashMap<(usize, usize), usize>,
}

impl Scanner {
    pub fn new(id: usize, input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        let mut beacons: HashMap<usize, Beacon> = HashMap::new();

        let mut number: usize = 0;

        for line in &lines[1..] {
            let l = *line;

            let coord: Vec<i64> = l.split(',').map(|x| x.parse::<i64>().unwrap()).collect();

            beacons.insert(number, Beacon::new(coord[0], coord[1], coord[2]));
            number += 1;
        }

        let mut dists: HashMap<(usize, usize), usize> = HashMap::new();

        for (n1, b1) in &beacons {
            for (n2, b2) in &beacons {
                if n1 < n2 {
                    dists.insert((*n1, *n2), Beacon::distance(b1, b2));
                }
            }
        }

        Scanner { id, beacons, dists }
    }
}

#[derive(Debug, Clone)]
pub struct Beacon {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Beacon {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn distance(a: &Self, b: &Self) -> usize {
        ((a.x - b.x) * (a.x - b.x)) as usize
            + ((a.y - b.y) * (a.y - b.y)) as usize
            + ((a.z - b.z) * (a.z - b.z)) as usize
    }
}
