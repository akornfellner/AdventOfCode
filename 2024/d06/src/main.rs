use std::collections::HashSet;

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut map = vec![];
    let mut start = (0, 0);

    for (x, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (y, c) in line.chars().enumerate() {
            match c {
                '#' => row.push(Field::Obstacle),
                '^' => {
                    row.push(Field::Empty);
                    start = (x as i32, y as i32);
                }
                _ => row.push(Field::Empty),
            }
        }
        map.push(row);
    }

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut d = 0usize;
    let mut cur_pos = start;
    let mut visited = HashSet::new();
    visited.insert(start);

    loop {
        let (nx, ny) = next(&cur_pos, d, &directions);
        if nx < 0 || nx >= map.len() as i32 || ny < 0 || ny >= map[0].len() as i32 {
            break;
        } else if let Field::Obstacle = map[nx as usize][ny as usize] {
            d = (d + 1) % 4;
        } else {
            cur_pos = (nx, ny);
            visited.insert(cur_pos);
        }
    }

    let p1 = visited.len();

    let possibles = visited
        .iter()
        .filter(|&&x| x != start)
        .map(|&(x, y)| (x, y))
        .collect::<Vec<(i32, i32)>>();

    let mut p2 = 0;

    for &p in &possibles {
        map[p.0 as usize][p.1 as usize] = Field::Obstacle;
        let mut d = 0usize;
        let mut cur_pos = start;
        let mut c = 0;

        loop {
            let (nx, ny) = next(&cur_pos, d, &directions);

            if nx < 0 || nx >= map.len() as i32 || ny < 0 || ny >= map[0].len() as i32 {
                break;
            } else if c == map.len() * map[0].len() {
                p2 += 1;
                break;
            } else if let Field::Obstacle = map[nx as usize][ny as usize] {
                d = (d + 1) % 4;
            } else {
                cur_pos = (nx, ny);
                c += 1;
            }
        }
        map[p.0 as usize][p.1 as usize] = Field::Empty;
    }

    (p1, p2)
}

fn next((x, y): &(i32, i32), d: usize, directions: &[(i32, i32)]) -> (i32, i32) {
    let (dx, dy) = directions[d];
    (*x + dx, *y + dy)
}

enum Field {
    Empty,
    Obstacle,
}
