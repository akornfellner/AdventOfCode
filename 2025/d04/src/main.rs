use std::env::args;

fn main() {
    let filename = args().nth(1).unwrap_or("input_test.txt".to_string());
    let (p1, p2) = solve(&filename);
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();

    let mut grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| matches!(c, '@'))
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    let mut p1 = 0;
    let mut p2 = 0;

    let mut finished = false;
    let mut iteration = 0;

    while !finished {
        finished = true;

        let mut delete = vec![];

        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                if grid[x][y] {
                    let neighbors = count_neighbors(x, y, &grid);
                    if neighbors.len() < 4 {
                        if iteration == 0 {
                            p1 += 1;
                        }
                        delete.push((x, y));
                        finished = false;
                    }
                }
            }
        }
        for (x, y) in delete {
            p2 += 1;
            grid[x][y] = false;
        }
        iteration += 1;
    }

    println!("Total iterations: {}", iteration);

    let final_grid = print_grid(&grid);
    std::fs::write("final_grid.txt", final_grid).unwrap();

    (p1, p2)
}

fn count_neighbors(x: usize, y: usize, grid: &[Vec<bool>]) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for (dx, dy) in directions.iter() {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0
            && ny >= 0
            && (nx as usize) < grid.len()
            && (ny as usize) < grid[0].len()
            && grid[nx as usize][ny as usize]
        {
            neighbors.push((nx as usize, ny as usize));
        }
    }

    neighbors
}

fn print_grid(grid: &[Vec<bool>]) -> String {
    let mut result = String::new();
    for row in grid {
        for &cell in row {
            result.push(if cell { '@' } else { '.' });
        }
        result.push('\n');
    }
    result
}
