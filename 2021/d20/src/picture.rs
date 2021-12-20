use std::fmt;

#[derive(Debug, Clone)]
pub struct Picture {
    pub pixels: Vec<Vec<bool>>,
}

impl Picture {
    pub fn new(field: &str) -> Self {
        let field: Vec<&str> = field.lines().collect();

        let mut pixels: Vec<Vec<bool>> = vec![vec![false; field[0].len() + 2]];

        for line in &field {
            let mut row: Vec<bool> = vec![false];
            for c in line.chars() {
                row.push(matches!(c, '#'));
            }
            row.push(false);
            pixels.push(row);
        }

        pixels.push(vec![false; field[0].len() + 2]);

        Picture { pixels }
    }

    pub fn calculate(&mut self, algo: &[bool], rounds: usize) {
        let switch = algo[0];

        for round in 0..rounds {
            let outside = round % 2 == 1 && switch;

            self.expand(outside);

            let p = self.pixels.clone();

            for i in 0..p.len() {
                for j in 0..p[0].len() {
                    let mut s = String::new();
                    for m in i as i32 - 1..=i as i32 + 1 {
                        for n in j as i32 - 1..=j as i32 + 1 {
                            let c;

                            if m < 0 || m >= p.len() as i32 || n < 0 || n >= p[0].len() as i32 {
                                if outside {
                                    c = "1";
                                } else {
                                    c = "0";
                                }
                            } else {
                                let (m, n) = (m as usize, n as usize);
                                c = match p[m][n] {
                                    true => "1",
                                    false => "0",
                                }
                            };
                            s += c;
                        }
                    }

                    let new = algo[usize::from_str_radix(&s, 2).unwrap()];
                    self.pixels[i][j] = new;
                }
            }
        }
    }

    fn expand(&mut self, with: bool) {
        let mut new: Vec<Vec<bool>> = vec![vec![with; self.pixels[0].len() + 2]];
        for row in &self.pixels {
            let mut new_row: Vec<bool> = vec![with];
            for b in row {
                new_row.push(*b);
            }
            new_row.push(with);
            new.push(new_row);
        }
        new.push(vec![with; self.pixels[0].len() + 2]);

        self.pixels = new;
    }

    pub fn count(&self) -> usize {
        let mut sum = 0usize;
        for row in &self.pixels {
            sum += row.iter().map(|x| *x as usize).sum::<usize>();
        }

        sum
    }
}

impl fmt::Display for Picture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        for i in 0..self.pixels.len() {
            for j in 0..self.pixels[0].len() {
                let b = self.pixels[i][j];
                output += match b {
                    true => "#",
                    false => ".",
                }
            }
            if i != self.pixels.len() - 1 {
                output += "\n";
            }
        }

        write!(f, "{}", output)
    }
}
