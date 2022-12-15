use std::fs;

pub fn solve(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let mut seats: Vec<Vec<Seat>> = vec![];

    for line in lines {
        let mut tmp: Vec<Seat> = vec![];

        for c in line.chars() {
            let seat: Seat = match c {
                'L' => Seat::Empty,
                '.' => Seat::Floor,
                _ => Seat::Occupied,
            };

            tmp.push(seat);
        }
        seats.push(tmp);
    }

    let mut seats_new = seats.clone();

    loop {
        let mut count = 0;

        for i in 0..seats.len() {
            for j in 0..seats[i].len() {
                let occupied = get_neighbours((i, j), &seats);

                match seats[i][j] {
                    Seat::Empty => {
                        if occupied == 0 {
                            seats_new[i][j] = Seat::Occupied;
                            count += 1;
                        }
                    }
                    Seat::Occupied => {
                        if occupied >= 5 {
                            seats_new[i][j] = Seat::Empty;
                            count += 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        seats = seats_new.clone();

        if count == 0 {
            break;
        }
    }

    count_occupied(&seats)
}

fn count_occupied(seats: &[Vec<Seat>]) -> usize {
    let mut count = 0usize;

    for line in seats {
        for seat in line {
            if let Seat::Occupied = seat {
                count += 1;
            }
        }
    }

    count
}

fn get_neighbours((x, y): (usize, usize), seats: &[Vec<Seat>]) -> usize {
    let mut occupied = 0usize;

    let dirs = [
        Direction::Left,
        Direction::LeftUp,
        Direction::Up,
        Direction::RightUp,
        Direction::Right,
        Direction::RightDown,
        Direction::Down,
        Direction::LeftDown,
    ];

    for dir in dirs {
        if let Seat::Occupied = check_line((x, y), seats, dir) {
            occupied += 1;
        }
    }

    occupied
}

fn check_line((x, y): (usize, usize), seats: &[Vec<Seat>], direction: Direction) -> Seat {
    let add_x: i32 = match direction {
        Direction::Up | Direction::LeftUp | Direction::RightUp => -1,
        Direction::Left | Direction::Right => 0,
        Direction::Down | Direction::LeftDown | Direction::RightDown => 1,
    };

    let add_y: i32 = match direction {
        Direction::Left | Direction::LeftUp | Direction::LeftDown => -1,
        Direction::Right | Direction::RightUp | Direction::RightDown => 1,
        Direction::Up | Direction::Down => 0,
    };

    let mut cur_x = x as i32;
    let mut cur_y = y as i32;

    let mut seat = Seat::Floor;

    loop {
        cur_x += add_x;
        cur_y += add_y;

        if cur_x < 0 || cur_x >= seats.len() as i32 || cur_y < 0 || cur_y >= seats[0].len() as i32 {
            break;
        }

        match seats[cur_x as usize][cur_y as usize] {
            Seat::Floor => {}
            _ => {
                seat = seats[cur_x as usize][cur_y as usize].clone();
                break;
            }
        }
    }

    seat
}

#[derive(Debug, Clone)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    LeftUp,
    Up,
    RightUp,
    Right,
    RightDown,
    Down,
    LeftDown,
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt"), 26);
    }
}
