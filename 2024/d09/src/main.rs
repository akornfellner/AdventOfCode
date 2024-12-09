fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();

    let mut free = vec![];
    let mut full = vec![];

    let mut size = 0;

    for (i, c) in input.chars().enumerate() {
        let c = c as usize - '0' as usize;
        for j in 0..c {
            if i % 2 == 0 {
                full.push(i / 2);
            } else {
                free.push(size + j);
            }
        }
        size += c;
    }

    while !free.is_empty() {
        let next = free.remove(0);
        if next >= full.len() {
            break;
        } else {
            let new = full.pop().unwrap();
            full.insert(next, new);
        }
    }

    let p1 = full.iter().enumerate().map(|(i, x)| i * x).sum::<usize>();

    let mut filesystem = vec![];

    for (i, c) in input.chars().enumerate() {
        let c = c as usize - '0' as usize;
        if i % 2 == 0 {
            filesystem.push(Space::File(c, i / 2));
        } else if c != 0 {
            filesystem.push(Space::Empty(c));
        }
    }

    let mut finished = false;
    let mut take = filesystem.len();

    while !finished {
        match step(&mut filesystem, take) {
            Some(t) => {
                take = t;
            }
            None => {
                finished = true;
            }
        }
    }

    (p1, checksum(&filesystem))
}

#[derive(Debug, Clone)]
enum Space {
    Empty(usize),
    File(usize, usize),
}

fn step(filesystem: &mut Vec<Space>, take: usize) -> Option<usize> {
    for (i, f) in filesystem.iter().enumerate().take(take).rev() {
        let f = f.clone();
        if let Space::File(size_f, _) = f {
            for (j, e) in filesystem.iter().enumerate().take(i) {
                let e = e.clone();
                if let Space::Empty(size_e) = e {
                    if size_e >= size_f {
                        filesystem.remove(i);
                        filesystem.insert(i, Space::Empty(size_f));
                        filesystem.remove(j);
                        filesystem.insert(j, f);
                        let mut add = 0;
                        if size_e > size_f {
                            filesystem.insert(j + 1, Space::Empty(size_e - size_f));
                            add = 1;
                        }
                        return Some(i + add);
                    }
                }
            }
        }
    }
    None
}

fn checksum(filesystem: &[Space]) -> usize {
    let mut s = 0;
    let mut offset = 0;
    for e in filesystem {
        match e {
            Space::Empty(size) => {
                offset += size;
            }
            Space::File(size, id) => {
                for j in 0..*size {
                    s += (offset + j) * id;
                }
                offset += size;
            }
        }
    }
    s
}
