use std::collections::HashSet;

fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    let mut galaxies: Vec<(usize, usize)> = vec![];
    let mut drows: HashSet<usize> = HashSet::new();
    let mut tmp: HashSet<usize> = HashSet::new();

    let mut cols = 0;

    for (x, line) in input.lines().enumerate() {
        let mut found = false;
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((x, y));
                tmp.insert(y);
                found = true;
            }
            cols = y;
        }
        if !found {
            drows.insert(x);
        }
    }

    let mut dcols: HashSet<usize> = (0..cols).collect();
    dcols = dcols.difference(&tmp).copied().collect();

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            result.0 += get_distance(galaxies[i], galaxies[j], &drows, &dcols, false);
            result.1 += get_distance(galaxies[i], galaxies[j], &drows, &dcols, true);
        }
    }

    result
}

fn get_distance(
    g1: (usize, usize),
    g2: (usize, usize),
    drows: &HashSet<usize>,
    dcols: &HashSet<usize>,
    two: bool,
) -> usize {
    let (x1, y1) = g1;
    let (x2, y2) = g2;
    let dx = x2 as i32 - x1 as i32;
    let dy = y2 as i32 - y1 as i32;

    let mut crows = 0;
    let mut ccols = 0;

    let (xmin, xmax) = if dx > 0 { (x1, x2) } else { (x2, x1) };
    let (ymin, ymax) = if dy > 0 { (y1, y2) } else { (y2, y1) };

    for row in drows {
        if *row > xmin && *row < xmax {
            crows += 1;
        }
    }

    for col in dcols {
        if *col > ymin && *col < ymax {
            ccols += 1;
        }
    }

    let m = if two { 999999 } else { 1 };

    (dx.abs() + dy.abs() + (ccols + crows) * m) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve("input_test.in").0, 374);
    }

    #[test]
    fn test_two() {
        assert_eq!(solve("input_test.in").1, 0);
    }
}
