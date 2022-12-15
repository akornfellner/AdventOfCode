use std::fmt;

pub fn solve(input: &str, rounds: usize) -> String {
    let mut pw = Password::new(input);

    for _ in 0..rounds {
        pw.next_valid();
    }

    pw.to_string()
}

#[derive(Debug)]
struct Password {
    pw: [u8; 8],
}

impl Password {
    fn new(input: &str) -> Self {
        if input.len() != 8 {
            panic!("password has not length 8");
        }

        let chars: Vec<char> = input.chars().collect();

        let mut pw: [u8; 8] = [0; 8];

        for i in 0..8 {
            pw[i] = chars[i] as u8 - 97;
        }

        Password { pw }
    }

    fn next_valid(&mut self) {
        loop {
            self.next();
            if self.is_valid() {
                break;
            }
        }
    }

    fn next(&mut self) {
        let mut index = 7;

        let mut finished = false;

        while !finished {
            self.pw[index] = Self::next_number(self.pw[index]);
            if self.pw[index] == 0 && index >= 1 {
                index -= 1;
            } else {
                finished = true;
            }
        }
    }

    fn next_number(n: u8) -> u8 {
        let new;
        if n == 7 || n == 10 || n == 13 {
            new = n + 2;
        } else {
            new = n + 1;
        }

        new % 26
    }

    fn is_valid(&self) -> bool {
        for sign in self.pw {
            if sign == 8 || sign == 11 || sign == 14 {
                return false;
            }
        }

        let mut three = false;
        let mut first = false;
        let mut second = false;

        for i in 0..self.pw.len() - 2 {
            if self.pw[i] + 1 == self.pw[i + 1] && self.pw[i] + 2 == self.pw[i + 2] {
                three = true;
            }
        }

        let mut index = 0usize;

        for i in 0..self.pw.len() - 1 {
            if (self.pw[i] == self.pw[i + 1])
                && (i == 0 || self.pw[i - 1] != self.pw[i])
                && (i == self.pw.len() - 2 || self.pw[i + 2] != self.pw[i])
            {
                first = true;
                index = i + 2;
                break;
            }
        }

        if first {
            for i in index..self.pw.len() - 1 {
                if (self.pw[i] == self.pw[i + 1])
                    && (i == 0 || self.pw[i - 1] != self.pw[i])
                    && (i == self.pw.len() - 2 || self.pw[i + 2] != self.pw[i])
                {
                    second = true;
                    break;
                }
            }
        }

        three && first && second
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut password = String::new();

        for sign in self.pw {
            password.push((sign + 97) as char);
        }
        write!(f, "{}", password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pw_fail_i() {
        let pw = Password::new("hijklmmn");
        assert!(!pw.is_valid());
    }

    #[test]
    fn pw_fail_three() {
        let pw = Password::new("abbceffg");
        assert!(!pw.is_valid());
    }

    #[test]
    fn pw_fail_one_double() {
        let pw = Password::new("abccdgjk");
        assert!(!pw.is_valid());
    }

    #[test]
    fn pw_fail_three_equal() {
        let pw = Password::new("abcfffaa");
        assert!(!pw.is_valid());
    }

    #[test]
    fn pw_valid() {
        let pw = Password::new("abcdffaa");
        assert!(pw.is_valid());
    }

    #[test]
    fn next_pw_1() {
        let mut pw = Password::new("abcdefgh");
        pw.next();
        assert_eq!(pw.to_string(), "abcdefgj".to_string());
    }

    #[test]
    fn it_works_1() {
        let mut pw = Password::new("abcdefgh");
        pw.next_valid();
        assert_eq!(pw.to_string(), "abcdffaa".to_string());
    }

    #[test]
    fn it_works_2() {
        let mut pw = Password::new("ghijklmn");
        pw.next_valid();
        assert_eq!(pw.to_string(), "ghjaabcc".to_string());
    }
}
