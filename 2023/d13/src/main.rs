fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);
    let patterns = input
        .split("\n\n")
        .map(Pattern::from)
        .collect::<Vec<Pattern>>();

    let mut twos: [usize; 17] = [0; 17];

    for (i, two) in twos.iter_mut().enumerate() {
        *two = 2usize.pow(i as u32);
    }

    for pattern in &patterns {
        let (r1, c1) = pattern.get_symmetry(&twos, false);
        result.0 += r1 * 100 + c1;
        let (r2, c2) = pattern.get_symmetry(&twos, true);
        result.1 += r2 * 100 + c2;
    }

    result
}

#[derive(Debug)]
struct Pattern {
    rows: Vec<usize>,
    cols: Vec<usize>,
    rowsigns: Vec<Vec<char>>,
    colsigns: Vec<Vec<char>>,
}

impl Pattern {
    fn from(input: &str) -> Self {
        let mut rows = vec![];
        let mut cols = vec![0; input.lines().next().unwrap().len()];
        let mut rowsigns: Vec<Vec<char>> = vec![];
        let mut colsigns: Vec<Vec<char>> = vec![vec![]; input.lines().next().unwrap().len()];

        let mut ml = 1usize;

        for line in input.lines() {
            let chars = line.chars().collect::<Vec<char>>();
            rowsigns.push(chars.clone());
            let mut row = 0usize;
            let mut mr = 1usize;
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    row += mr;
                    cols[j] += ml;
                    colsigns[j].push('#');
                } else {
                    colsigns[j].push('.');
                }
                mr *= 2;
            }
            ml *= 2;
            rows.push(row);
        }

        Self {
            rows,
            cols,
            rowsigns,
            colsigns,
        }
    }

    fn rows_one_diff_sign(first: &[char], second: &[char]) -> bool {
        let mut diff = 0usize;
        for (i, c) in first.iter().enumerate() {
            if c != &second[i] {
                diff += 1;
            }
        }
        diff == 1
    }

    fn find_symmetry(list: &[usize], signs: &[Vec<char>], twos: &[usize], two: bool) -> usize {
        let mut i = 0usize;
        let mut is_symmetric = false;
        let mut is_toggled = false;

        while (two && !is_toggled || !is_symmetric) && i < list.len() - 1 {
            for j in i + 1..list.len() {
                let diff = (list[j - 1] as i32 - list[j] as i32).unsigned_abs() as usize;
                if list[j - 1] == list[j] || twos.contains(&diff) && two {
                    is_symmetric = true;
                    i = j - 1;
                    break;
                }
            }

            if is_symmetric {
                let mut min = i as i32;
                let mut max = i + 1;

                while min >= 0 && max < list.len() {
                    let diff =
                        (list[min as usize] as i32 - list[max] as i32).unsigned_abs() as usize;

                    if diff != 0
                        && twos.contains(&diff)
                        && !is_toggled
                        && two
                        && Self::rows_one_diff_sign(&signs[min as usize], &signs[max])
                    {
                        is_toggled = true;
                    } else if diff != 0 {
                        is_toggled = false;
                        is_symmetric = false;
                        break;
                    }
                    min -= 1;
                    max += 1;
                }
            }
            i += 1;
        }

        if !is_symmetric {
            i = 0;
        }

        if two && !is_toggled {
            i = 0;
        }

        i
    }

    fn get_symmetry(&self, twos: &[usize], two: bool) -> (usize, usize) {
        (
            Self::find_symmetry(&self.rows, &self.rowsigns, twos, two),
            Self::find_symmetry(&self.cols, &self.colsigns, twos, two),
        )
    }
}
