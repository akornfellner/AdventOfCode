use std::cmp::Ordering;

fn main() {
    let (p1, p2) = solve("input.txt", (101, 103));
    //let (p1, p2) = solve("input_test.txt", (11, 7));
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str, field: (i32, i32)) -> (i32, i32) {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();

    let mut robots = input.lines().map(Robot::from).collect::<Vec<_>>();

    let mut r = robots.clone();

    r.iter_mut().for_each(|robot| robot.step(field, 100));

    let mut quadrants = [0; 4];

    for robot in &r {
        let x_cmp = robot.p.0.cmp(&(field.0 / 2));
        let y_cmp = robot.p.1.cmp(&(field.1 / 2));

        let q = match (x_cmp, y_cmp) {
            (Ordering::Less, Ordering::Less) => 0,
            (Ordering::Less, Ordering::Greater) => 2,
            (Ordering::Greater, Ordering::Less) => 1,
            (Ordering::Greater, Ordering::Greater) => 3,
            _ => -1,
        };

        if q != -1 {
            quadrants[q as usize] += 1;
        }
    }

    let mut r = robots.clone();

    let (mut min_x_v, mut min_y_v) = variance(&r);
    let (mut x_steps, mut y_steps) = (0, 0);

    for i in 1..103 {
        r.iter_mut().for_each(|robot| robot.step(field, 1));
        let (var_x, var_y) = variance(&r);
        if var_x < min_x_v {
            min_x_v = var_x;
            x_steps = i;
        }
        if var_y < min_y_v {
            min_y_v = var_y;
            y_steps = i;
        }
    }

    let mut p2 = 0;

    for t in (x_steps..).step_by(field.0 as usize) {
        if (t - y_steps) % field.1 == 0 {
            p2 = t;
            break;
        }
    }

    robots.iter_mut().for_each(|robot| robot.step(field, p2));

    print_field(&robots, field);

    (quadrants.iter().product(), p2)
}

#[derive(Debug, Clone)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

impl Robot {
    fn from(line: &str) -> Self {
        let line = line.replace(['=', ','], " ");
        let parts = line.split_whitespace().collect::<Vec<_>>();

        let p = (parts[1].parse().unwrap(), parts[2].parse().unwrap());
        let v = (parts[4].parse().unwrap(), parts[5].parse().unwrap());

        Self { p, v }
    }

    fn step(&mut self, field: (i32, i32), steps: i32) {
        self.p.0 = (self.p.0 + self.v.0 * steps).rem_euclid(field.0);
        self.p.1 = (self.p.1 + self.v.1 * steps).rem_euclid(field.1);
    }
}

fn print_field(robots: &[Robot], field: (i32, i32)) {
    let mut field = vec![vec!['.'; field.0 as usize]; field.1 as usize];

    for robot in robots {
        field[robot.p.1 as usize][robot.p.0 as usize] = '#';
    }

    let result = field
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", result);
}

fn variance(robots: &[Robot]) -> (f64, f64) {
    let x = robots
        .iter()
        .map(|robot| robot.p.0 as f64)
        .collect::<Vec<_>>();
    let y = robots
        .iter()
        .map(|robot| robot.p.1 as f64)
        .collect::<Vec<_>>();

    let mean_x = x.iter().sum::<f64>() / x.len() as f64;
    let mean_y = y.iter().sum::<f64>() / y.len() as f64;
    (
        x.iter().map(|x| (x - mean_x).powi(2)).sum::<f64>() / x.len() as f64,
        y.iter().map(|y| (y - mean_y).powi(2)).sum::<f64>() / y.len() as f64,
    )
}
