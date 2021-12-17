pub fn solve_one(min_y: i32) -> i32 {
    let y = min_y.abs() - 1;
    y * (y + 1) / 2
}

pub fn solve_two(min: (i32, i32), max: (i32, i32)) -> i32 {
    let mut count = 0;

    for x in 1..=max.0 {
        for y in min.1..=min.1.abs() - 1 {
            if try_vel(x, y, min, max) {
                count += 1;
            }
        }
    }

    count
}

fn try_vel(mut x: i32, mut y: i32, min: (i32, i32), max: (i32, i32)) -> bool {
    let mut x_pos = 0;
    let mut y_pos = 0;

    loop {
        x_pos += x;
        y_pos += y;

        if x_pos > max.0 || y_pos < min.1 {
            break;
        }

        if x_pos >= min.0 && x_pos <= max.0 && y_pos >= min.1 && y_pos <= max.1 {
            return true;
        }

        if x > 0 {
            x -= 1;
        }
        y -= 1;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::{solve_one, solve_two};

    #[test]
    fn one_works() {
        assert_eq!(solve_one(-10), 45);
    }

    #[test]
    fn two_works() {
        assert_eq!(solve_two((20, -10), (30, -5)), 112);
    }
}
