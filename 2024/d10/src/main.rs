use std::collections::HashSet;

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let map: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as i32 - '0' as i32).collect())
        .collect();

    let (mut p1, mut p2) = (0, 0);

    for (i, row) in map.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if *value == 0 {
                let mut visited = HashSet::new();
                p2 += find_path(i, j, &map, &mut visited);
                p1 += visited.len();
            }
        }
    }

    (p1, p2)
}

fn find_path(x: usize, y: usize, map: &[Vec<i32>], visited: &mut HashSet<(usize, usize)>) -> usize {
    if map[x][y] == 9 {
        visited.insert((x, y));
        return 1;
    }

    let x = x as i32;
    let y = y as i32;
    let mut result = 0;

    for (i, j) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
        if i >= 0
            && j >= 0
            && i < map.len() as i32
            && j < map[0].len() as i32
            && map[i as usize][j as usize] - map[x as usize][y as usize] == 1
        {
            result += find_path(i as usize, j as usize, map, visited);
        }
    }
    result
}
