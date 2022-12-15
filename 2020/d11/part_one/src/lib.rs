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
                        if occupied >= 4 {
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

fn get_neighbours((x, y): (usize, usize), seats: &Vec<Vec<Seat>>) -> usize {
    let mut start_x = x as i32 - 1;
    let mut end_x = x as i32 + 1;
    let mut start_y = y as i32 - 1;
    let mut end_y = y as i32 + 1;

    if x == 0 {
        start_x = x as i32;
    }
    if y == 0 {
        start_y = y as i32;
    }
    if x == seats.len() - 1 {
        end_x = x as i32;
    }
    if y == seats[0].len() - 1 {
        end_y = y as i32;
    }

    let mut occupied = 0usize;

    for i in start_x as usize..=end_x as usize {
        for j in start_y as usize..=end_y as usize {
            if !(i == x && j == y) {
                if let Seat::Occupied = seats[i][j] {
                    occupied += 1;
                }
            }
        }
    }

    occupied
}

#[derive(Debug, Clone)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt"), 37);
    }
}
