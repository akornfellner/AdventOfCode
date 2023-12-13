fn main() {
    let (p1, p2) = solve("input_test.in");
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

    // for pattern in &patterns {
    //     let (r, c) = pattern.get_symmetry();
    //     result.0 += r * 100 + c;
    // }

    println!("{:?}", patterns[1].get_symmetry1(&twos));
    println!("{:?}", patterns[1].get_symmetry2(&twos));

    result
}

#[derive(Debug)]
struct Pattern {
    rows: Vec<usize>,
    cols: Vec<usize>,
}

impl Pattern {
    fn from(input: &str) -> Self {
        let mut rows = vec![];
        let mut cols = vec![0; input.lines().next().unwrap().len()];

        let mut ml = 1usize;

        for line in input.lines() {
            let mut row = 0usize;
            let mut mr = 1usize;
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    row += mr;
                    cols[j] += ml;
                }
                mr *= 2;
            }
            ml *= 2;
            rows.push(row);
        }

        Self { rows, cols }
    }

    fn find_symmetry(list: &[usize]) -> usize {
        let mut i = 0usize;
        let mut is_symmetric = false;

        while !is_symmetric && i < list.len() - 1 {
            for j in i + 1..list.len() {
                if list[j - 1] == list[j] {
                    is_symmetric = true;
                    i = j - 1;
                    break;
                }
            }

            if is_symmetric {
                let mut min = i as i32;
                let mut max = i + 1;

                while min >= 0 && max < list.len() {
                    if list[min as usize] != list[max] {
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

        i
    }

    fn find_symmetry2(list: &[usize], twos: &[usize]) -> usize {
        let mut i = 0usize;
        let mut is_symmetric = false;

        while !is_symmetric && i < list.len() - 1 {
            let mut count = 0;
            for j in i + 1..list.len() {
                let diff = (list[j - 1] as i32 - list[j] as i32).abs() as usize;
                if list[j - 1] == list[j] {
                    is_symmetric = true;
                    i = j - 1;
                    break;
                } else if twos.contains(&diff) {
                    is_symmetric = true;
                    i = j - 1;
                    break;
                }
            }

            if is_symmetric {
                let mut min = i as i32;
                let mut max = i + 1;

                while min >= 0 && max < list.len() {
                    let diff = (list[min as usize] as i32 - list[max] as i32).abs() as usize;

                    if diff != 0 && twos.contains(&diff) && count == 0 {
                        count += 1;
                    } else if diff != 0 {
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

        i
    }

    fn get_symmetry1(&self, twos: &[usize]) -> (usize, usize) {
        (
            Self::find_symmetry(&self.rows),
            Self::find_symmetry(&self.cols),
        )
    }

    fn get_symmetry2(&self, twos: &[usize]) -> (usize, usize) {
        (
            Self::find_symmetry2(&self.rows, twos),
            Self::find_symmetry2(&self.cols, twos),
        )
    }
}
