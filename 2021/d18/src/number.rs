use std::{
    fmt,
    ops::{Add, Index},
};

#[derive(Debug, Clone)]
pub struct Number {
    pub arr: Vec<Entry>,
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
                let p = match c {
                    '[' => Entry::Open,
                    ']' => Entry::Close,
                    ',' => Entry::Comma,
                    _ => Entry::Comma,
                };
                arr.push(p);
            } else {
                active_number = true;
                curr_number += &c.to_string();
            }
        }
        Number { arr }
    }

    pub fn get_parts(&self) -> (Self, Self) {
        let mut index = 0usize;
        let mut count = 0usize;
        for i in 1..self.len() {
            let e = self[i].clone();
            if let Entry::Open = e {
                count += 1;
            } else if let Entry::Close = e {
                count -= 1;
            } else if let Entry::Comma = e {
                if count == 0 {
                    index = i;
                    break;
                }
            }
        }

        let left_number = Number {
            arr: self.arr[1..index].to_owned(),
        };

        let right_number = Number {
            arr: self.arr[index + 1..self.arr.len() - 1].to_owned(),
        };

        (left_number, right_number)
    }

    fn is_number(&self) -> bool {
        if self.len() > 1 {
            return false;
        }
        true
    }

    fn len(&self) -> usize {
        self.arr.len()
    }

    pub fn magnitude(&self) -> usize {
        let (left, right) = self.get_parts();

        let (l, r);

        if left.is_number() {
            l = left.arr[0].get_value();
        } else {
            l = left.magnitude();
        }

        if right.is_number() {
            r = right.arr[0].get_value();
        } else {
            r = right.magnitude();
        }

        3 * l + 2 * r
    }

    pub fn reduce(number: Self) -> Self {
        let mut new = number;
        loop {
            let (exploded, number) = Self::explode(new);
            new = number;
            let mut splitted = false;
            if !exploded {
                let (s, number) = Self::split(new);
                splitted = s;
                new = number;
            }
            if !exploded && !splitted {
                break;
            }
        }
        new
    }

    fn explode(number: Number) -> (bool, Number) {
        let mut last_number = 0usize;
        let mut start = 0usize;
        let mut end = 0usize;

        let mut count = 0usize;
        let mut left = 0usize;
        let mut right = 0usize;
        let mut found = false;

        for i in 0..number.len() {
            let e = &number.arr[i];
            if let Entry::Open = e {
                count += 1;
            } else if let Entry::Close = e {
                count -= 1;
            } else if count > 4 && e.is_number() && number[i + 2].is_number() {
                start = i - 1;
                end = i + 3;
                left = e.get_value();
                right = number[i + 2].get_value();
                found = true;
                break;
            } else if e.is_number() {
                last_number = i;
            }
        }
        let mut new: Vec<Entry> = vec![];

        if found {
            if last_number != 0 {
                for c in &number.arr[..last_number] {
                    new.push(c.clone());
                }
                let new_left = number[last_number].get_value() + left;
                new.push(Entry::Number(new_left));
                for k in last_number + 1..start {
                    new.push(number[k].clone());
                }
            } else {
                for c in &number.arr[..start] {
                    new.push(c.clone());
                }
            }
            new.push(Entry::Number(0));
            let mut found_next = false;
            for k in &number.arr[end + 1..] {
                if !found_next && k.is_number() {
                    let n = k.get_value() + right;
                    new.push(Entry::Number(n));
                    found_next = true;
                } else {
                    new.push(k.clone());
                }
            }
        } else {
            return (false, number);
        }

        (true, Number { arr: new })
    }

    fn split(number: Number) -> (bool, Number) {
        let mut splitted = false;
        let mut new: Vec<Entry> = vec![];

        let mut index = 0usize;
        let mut value = 0usize;

        for i in 0..number.len() {
            let e = &number[i];
            if e.is_number() && e.get_value() > 9 {
                index = i;
                splitted = true;
                value = e.get_value();
                break;
            }
        }

        if splitted {
            for e in &number.arr[..index] {
                new.push(e.clone())
            }
            let (left, right);
            left = value / 2;
            if value % 2 == 0 {
                right = value / 2;
            } else {
                right = value / 2 + 1;
            }
            new.push(Entry::Open);
            new.push(Entry::Number(left));
            new.push(Entry::Comma);
            new.push(Entry::Number(right));
            new.push(Entry::Close);
            for e in &number.arr[index + 1..] {
                new.push(e.clone());
            }

            return (true, Number { arr: new });
        }

        (false, number)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        for c in &self.arr {
            let s = match c {
                Entry::Open => "[".to_string(),
                Entry::Comma => ",".to_string(),
                Entry::Close => "]".to_string(),
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
        let mut arr: Vec<Entry> = vec![Entry::Open];
        for e in self.arr {
            arr.push(e);
        }
        arr.push(Entry::Comma);
        for e in other.arr {
            arr.push(e);
        }
        arr.push(Entry::Close);

        Number { arr }
    }
}

impl Index<usize> for Number {
    type Output = Entry;
    fn index(&self, index: usize) -> &Self::Output {
        &self.arr[index]
    }
}

#[derive(Debug, Clone)]
pub enum Entry {
    Number(usize),
    Open,
    Close,
    Comma,
}

impl Entry {
    fn is_number(&self) -> bool {
        if let Entry::Number(_) = self {
            return true;
        }
        false
    }

    pub fn get_value(&self) -> usize {
        if let Entry::Number(value) = self {
            return *value;
        }
        0
    }
}

fn is_number(c: char) -> bool {
    if c as usize >= '0' as usize && c as usize <= '9' as usize {
        return true;
    }
    false
}
