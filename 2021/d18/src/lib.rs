mod number;

use std::fs;

use crate::number::{Entry, Number};

pub fn solve_one(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut number = Number::new(lines[0]);

    for line in &lines[1..] {
        number = number + Number::new(*line);
        number = reduce(number);
    }

    number.magnitude()
}

pub fn solve_two(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut max = 0usize;

    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i != j {
                let number = Number::new(lines[i]) + Number::new(lines[j]);
                let number = reduce(number);
                let mag = number.magnitude();
                if mag > max {
                    max = mag;
                }
            }
        }
    }

    max
}

fn reduce(number: Number) -> Number {
    let mut new = number.to_owned();
    loop {
        let (exploded, number) = explode(new);
        new = number;
        let mut splitted = false;
        if !exploded {
            let (s, number) = split(new);
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
        return (false, number.clone());
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

    (false, number.clone())
}

#[cfg(test)]
mod tests {
    use crate::number::Number;
    use crate::{solve_one, solve_two};

    #[test]
    fn one_works() {
        assert_eq!(solve_one("input_test.txt"), 4140);
    }

    #[test]
    fn two_works() {
        assert_eq!(solve_two("input_test.txt"), 3993);
    }

    #[test]
    fn mag_1() {
        let number = Number::new("[[1,2],[[3,4],5]]");
        assert_eq!(number.magnitude(), 143);
    }

    #[test]
    fn mag_2() {
        let number = Number::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(number.magnitude(), 1384);
    }

    #[test]
    fn mag_3() {
        let number = Number::new("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(number.magnitude(), 445);
    }

    #[test]
    fn mag_4() {
        let number = Number::new("[[[[3,0],[5,3]],[4,4]],[5,5]]");
        assert_eq!(number.magnitude(), 791);
    }

    #[test]
    fn mag_5() {
        let number = Number::new("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_eq!(number.magnitude(), 1137);
    }

    #[test]
    fn mag_6() {
        let number = Number::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(number.magnitude(), 3488);
    }
}
