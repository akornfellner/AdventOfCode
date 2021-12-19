use std::{fmt, ops::Add};

#[derive(Debug, Clone)]
pub struct Number {
    arr: Vec<Entry>,
}

impl Number {
    pub fn new(input: &str) -> Self {
        let mut arr: Vec<Entry> = vec![];

        let mut active_number = false;
        let mut curr_number = String::new();
        for c in input.chars() {
            if !is_number(c) {
                if active_number {
                    arr.push(Entry::Number(curr_number.parse::<usize>().unwrap()));
                    curr_number.clear();
                }
                active_number = false;
                arr.push(Entry::Char(c));
            } else {
                active_number = true;
                curr_number += &c.to_string();
            }
        }
        Number { arr }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        for c in &self.arr {
            let s = match c {
                Entry::Char(value) => value.to_string(),
                Entry::Number(value) => value.to_string(),
            };
            output += &s;
        }

        write!(f, "{}", output)
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut arr: Vec<Entry> = vec![];
        arr.push(Entry::Char('['));
        for e in self.arr {
            arr.push(e);
        }
        arr.push(Entry::Char(','));
        for e in other.arr {
            arr.push(e);
        }
        arr.push(Entry::Char(']'));

        Number { arr }
    }
}

#[derive(Debug, Clone)]
pub enum Entry {
    Number(usize),
    Char(char),
}

impl Entry {
    pub fn is_number(&self) -> bool {
        if let Entry::Number(_) = self {
            return true;
        }
        false
    }
}

fn is_number(c: char) -> bool {
    if c as usize >= '0' as usize && c as usize <= '9' as usize {
        return true;
    }
    false
}
