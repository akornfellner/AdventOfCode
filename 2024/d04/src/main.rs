fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();

    let field = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let dirs = vec![
        (1, 0),
        (0, 1),
        (1, 1),
        (1, -1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (-1, 1),
    ];

    let mut p1 = 0;
    let mut p2 = 0;

    for x in 0..field.len() {
        for y in 0..field[0].len() {
            for dir in &dirs {
                if field[x][y] == 'X' {
                    p1 += find_xmas(&field, (x, y), *dir);
                }
            }

            if x > 0 && x < field.len() - 1 && y > 0 && y < field[0].len() - 1 && field[x][y] == 'A'
            {
                p2 += find_mas(&field, (x, y));
            }
        }
    }

    (p1, p2)
}

fn find_xmas(field: &[Vec<char>], (x, y): (usize, usize), dir: (i32, i32)) -> usize {
    let xmas = ['X', 'M', 'A', 'S'];
    let mut x = x as i32;
    let mut y = y as i32;

    for c in &xmas {
        if x < 0 || x >= field.len() as i32 || y < 0 || y >= field[0].len() as i32 {
            return 0;
        }
        if field[x as usize][y as usize] != *c {
            return 0;
        }
        x += dir.0;
        y += dir.1;
    }
    1
}

fn find_mas(field: &[Vec<char>], (x, y): (usize, usize)) -> usize {
    let left = field[x - 1][y - 1] == 'S' && field[x + 1][y + 1] == 'M'
        || field[x - 1][y - 1] == 'M' && field[x + 1][y + 1] == 'S';
    let right = field[x + 1][y - 1] == 'S' && field[x - 1][y + 1] == 'M'
        || field[x + 1][y - 1] == 'M' && field[x - 1][y + 1] == 'S';
    if left && right {
        1
    } else {
        0
    }
}
