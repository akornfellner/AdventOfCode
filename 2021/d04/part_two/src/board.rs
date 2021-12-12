const BOARD_SIZE: usize = 5;

#[derive(Debug, Clone)]
pub struct Board {
    pub board: [[(i32, bool); BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new_from_string(input: &str) -> Self {
        let rows: Vec<&str> = input.split('\n').collect();

        let mut board = [[(0, false); BOARD_SIZE]; BOARD_SIZE];
        for i in 0..rows.len() {
            board[i] = Board::split_row(rows[i])
        }

        Board { board }
    }

    fn split_row(row: &str) -> [(i32, bool); 5] {
        let tmp: Vec<&str> = row.split(' ').collect();

        let mut cleaned: Vec<&str> = vec![];

        for s in tmp {
            if s.len() > 0 {
                cleaned.push(s);
            }
        }

        let values: Vec<i32> = cleaned
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut result = [(0, false); BOARD_SIZE];

        for i in 0..values.len() {
            result[i] = (values[i], false);
        }

        result
    }

    pub fn new_number(&mut self, number: i32) {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j].0 == number {
                    self.board[i][j].1 = true;
                }
            }
        }
    }

    pub fn won(&self) -> bool {
        for i in 0..BOARD_SIZE {
            let mut result = true;
            for j in 0..BOARD_SIZE {
                result = result && self.board[i][j].1;
            }
            if result {
                return result;
            }
        }

        for j in 0..BOARD_SIZE {
            let mut result = true;
            for i in 0..BOARD_SIZE {
                result = result && self.board[i][j].1;
            }
            if result {
                return result;
            }
        }

        false
    }

    pub fn sum_unmarked(&self) -> i32 {
        let mut sum = 0;
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.board[i][j].1 == false {
                    sum += self.board[i][j].0
                }
            }
        }
        sum
    }
}
