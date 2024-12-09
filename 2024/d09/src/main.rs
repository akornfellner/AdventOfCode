fn main() {
    let start = std::time::Instant::now();
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
    println!("Time: {}ms", start.elapsed().as_millis());
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();

    let mut free = Vec::new();
    let mut full = Vec::new();

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

    while free.len() > 0 {
        let next = free.remove(0);
        if next >= full.len() {
            break;
        } else {
            let new = full.pop().unwrap();
            full.insert(next, new);
        }
    }

    let p1 = full.iter().enumerate().map(|(i, x)| i * x).sum::<usize>();

    let mut filesystem = Vec::new();

    for (i, c) in input.chars().enumerate() {
        let c = c as usize - '0' as usize;
        if i % 2 == 0 {
            filesystem.push(Space::File(c, i / 2));
        } else {
            if c != 0 {
                filesystem.push(Space::Empty(c));
            }
        }
    }

    loop {
        if !step(&mut filesystem) {
            break;
        }
    }

    (p1, checksum(&filesystem))
}

#[derive(Debug, Clone)]
enum Space {
    Empty(usize),
    File(usize, usize),
}

fn step(filesystem: &mut Vec<Space>) -> bool {
    for (i, f) in filesystem.iter().enumerate().rev() {
        let f = f.clone();
        if let Space::File(size_f, _) = f {
            for (j, e) in filesystem.iter().enumerate() {
                let e = e.clone();
                if let Space::Empty(size_e) = e {
                    if j < i && size_e >= size_f {
                        filesystem.remove(i);
                        filesystem.insert(i, Space::Empty(size_f));
                        filesystem.remove(j);
                        filesystem.insert(j, f);
                        if size_e > size_f {
                            filesystem.insert(j + 1, Space::Empty(size_e - size_f));
                        }
                        return true;
                    }
                }
            }
        }
    }
    false
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
