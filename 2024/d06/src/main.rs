use std::collections::HashSet;

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut obstacles: Vec<(i32, i32)> = vec![];
    let max_x = lines.len() as i32;
    let max_y = lines[0].len() as i32;
    let mut start = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                obstacles.push((i as i32, j as i32));
            } else if c == '^' {
                start = (i as i32, j as i32);
            }
        }
    }

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut d = 0usize;
    let mut cur_pos = start;
    let mut visited = HashSet::new();
    visited.insert(start);

    loop {
        let (nx, ny) = next(&cur_pos, d, &directions);
        if nx < 0 || nx >= max_x || ny < 0 || ny >= max_y {
            break;
        } else if obstacles.contains(&(nx, ny)) {
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
        d = 0;
        cur_pos = start;
        let mut visited = HashSet::new();
        visited.insert((cur_pos, d));
        let mut new_obstacles = obstacles.to_vec();
        new_obstacles.push(p);

        loop {
            let (nx, ny) = next(&cur_pos, d, &directions);
            let n = (nx, ny);
            if nx < 0 || nx >= max_x || ny < 0 || ny >= max_y {
                break;
            } else if visited.contains(&(n, d)) {
                p2 += 1;
                break;
            } else if new_obstacles.contains(&n) {
                d = (d + 1) % 4;
            } else {
                cur_pos = (nx, ny);
                visited.insert((cur_pos, d));
            }
        }
    }

    (p1, p2)
}

fn next((x, y): &(i32, i32), d: usize, directions: &[(i32, i32)]) -> (i32, i32) {
    let (dx, dy) = directions[d];
    (*x + dx, *y + dy)
}
