use std::{fmt::Display, fs};

pub struct Field {
    lights: Vec<Vec<bool>>,
}

impl Field {
    pub fn from(input: &str) -> Self {
        let input = fs::read_to_string(input).unwrap();
        let lines: Vec<&str> = input.lines().collect();

        let mut field: Vec<Vec<bool>> = vec![];

        for l in lines {
            let mut line: Vec<bool> = vec![];

            for c in l.chars() {
                match c {
                    '#' => line.push(true),
                    _ => line.push(false),
                }
            }

            field.push(line);
        }

        Field { lights: field }
    }

    pub fn rows(&self) -> usize {
        self.lights.len()
    }

    pub fn cols(&self) -> usize {
        self.lights[0].len()
    }

    pub fn set_light(&mut self, x: usize, y: usize, value: bool) {
        self.lights[x][y] = value;
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours: Vec<(usize, usize)> = vec![];

        let mut start_i = -1;
        let mut end_i = 1;
        let mut start_j = -1;
        let mut end_j = 1;

        if x == 0 {
            start_i = 0;
        }

        if y == 0 {
            start_j = 0;
        }

        if x == self.lights.len() - 1 {
            end_i = 0;
        }

        if y == self.lights[x].len() - 1 {
            end_j = 0;
        }

        for i in start_i..=end_i {
            for j in start_j..=end_j {
                if !(i == 0 && j == 0) {
                    let a = (x as i32 + i) as usize;
                    let b = (y as i32 + j) as usize;
                    neighbours.push((a, b));
                }
            }
        }

        neighbours
    }

    fn count_neighbours(&self, x: usize, y: usize) -> usize {
        let neighbours = self.get_neighbours(x, y);
        let mut count: usize = 0;

        for (i, j) in neighbours {
            if self.lights[i][j] {
                count += 1;
            }
        }

        count
    }

    fn is_corner(&self, x: usize, y: usize) -> bool {
        x == 0 && (y == 0 || y == self.lights[x].len() - 1)
            || x == self.lights.len() - 1 && (y == 0 || y == self.lights[x].len() - 1)
    }

    pub fn round(self, two: bool) -> Self {
        let mut new = self.lights.clone();

        for (i, line) in self.lights.iter().enumerate() {
            for (j, light) in line.iter().enumerate() {
                let count = self.count_neighbours(i, j);

                if !(two && self.is_corner(i, j)) {
                    if *light {
                        match count {
                            2 | 3 => (),
                            _ => new[i][j] = false,
                        }
                    } else if count == 3 {
                        new[i][j] = true;
                    }
                }
            }
        }

        Field { lights: new }
    }

    pub fn count_lights(&self) -> usize {
        let mut count = 0usize;

        for line in &self.lights {
            for light in line {
                if *light {
                    count += 1;
                }
            }
        }

        count
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for line in &self.lights {
            for value in line {
                if *value {
                    result += "#";
                } else {
                    result += ".";
                }
            }
            result += "\n";
        }

        write!(f, "{result}")
    }
}
