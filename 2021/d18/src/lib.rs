mod number;

use std::fs;

use crate::number::{Entry, Number};

pub fn solve(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let input = "[[9,8],7]";
    let input2 = "[1,1]";

    let a = Number::new(input);
    let b = Number::new(input2);

    let c = Entry::Char('c').is_number();
    println!("{}", c);

    println!("{}", a + b);

    5
}

// fn add(first: &str, second: &str) -> String {
//     let mut result = String::from("[");
//     result += first;
//     result += ",";
//     result += second;
//     result += "]";

//     println!("{}", result);

//     reduce(&result.chars().collect::<Vec<char>>())
//         .iter()
//         .collect::<String>()
// }

// fn reduce(input: &[char]) -> Vec<char> {
//     let mut new = input.to_owned();
//     loop {
//         println!("{}", new.iter().collect::<String>());

//         let (exploded, v) = explode(&new);
//         new = v;
//         let mut splitted = false;
//         if !exploded {
//             let (s, v) = split(&new);
//             splitted = s;
//             new = v;
//         }
//         if !exploded && !splitted {
//             break;
//         }
//     }
//     new
// }

// fn explode(input: &[char]) -> (bool, Vec<char>) {
//     let mut last_number = 0usize;
//     let mut start = 0usize;
//     let mut end = 0usize;

//     let mut count = 0usize;
//     let mut left = 0usize;
//     let mut right = 0usize;

//     for i in 0..input.len() {
//         let c = input[i];
//         if c == '[' {
//             count += 1;
//         } else if c == ']' {
//             count -= 1;
//         } else if count > 4 && is_number(c) {
//             let (is_pair, l, r, s, e) = explode_pair(input, i);
//             if is_pair {
//                 left = l;
//                 right = r;
//                 start = s;
//                 end = e;
//                 break;
//             }
//         } else if is_number(c) {
//             last_number = i;
//         }
//     }
//     let mut new: Vec<char> = vec![];

//     if !(left == 0 && right == 0) {
//         if last_number != 0 {
//             for c in &input[..last_number] {
//                 new.push(*c);
//             }
//             let new_left = to_number(input[last_number]) + left;
//             for k in to_char(new_left) {
//                 new.push(k);
//             }
//             for k in last_number + 1..start {
//                 new.push(input[k]);
//             }
//         } else {
//             for c in &input[..start] {
//                 new.push(*c);
//             }
//         }
//         new.push('0');
//         let mut found_next = false;
//         for k in &input[end + 1..] {
//             if !found_next && is_number(*k) {
//                 let n = to_number(*k) + right;
//                 for j in to_char(n) {
//                     new.push(j);
//                 }
//                 found_next = true;
//             } else {
//                 new.push(*k);
//             }
//         }
//     } else {
//         return (false, input.to_owned());
//     }

//     (true, new)
// }

// fn explode_pair(input: &[char], index: usize) -> (bool, usize, usize, usize, usize) {
//     if input[index - 1] == ']' {
//         return (false, 0, 0, 0, 0);
//     }

//     let mut left = String::new();
//     let mut right = String::new();

//     let mut end = 0usize;

//     let mut infirst = true;

//     for i in index..input.len() {
//         let c = input[i];
//         if is_number(c) && infirst {
//             left += &c.to_string();
//         } else if c == ',' && infirst {
//             infirst = false;
//         } else if is_number(c) && !infirst {
//             right += &c.to_string();
//         } else if c == '[' {
//             return (false, 0, 0, 0, 0);
//         } else if c == ']' {
//             if infirst {
//                 return (false, 0, 0, 0, 0);
//             } else {
//                 end = i;
//                 break;
//             }
//         }
//     }
//     (
//         true,
//         left.parse::<usize>().unwrap(),
//         right.parse::<usize>().unwrap(),
//         index - 1,
//         end,
//     )
// }

// fn split(input: &[char]) -> (bool, Vec<char>) {
//     let mut start = 0usize;
//     let mut end = 0usize;
//     let mut left = 0usize;
//     let mut right = 0usize;

//     for i in 0..input.len() {
//         if is_number(input[i]) && is_number(input[i + 1]) {
//             let mut number = input[i].to_string();
//             number += &input[i + 1].to_string();
//             let number = number.parse::<usize>().unwrap();
//             if number % 2 == 0 {
//                 left = number / 2;
//                 right = number / 2;
//             } else {
//                 left = number / 2;
//                 right = number / 2 + 1;
//             }
//             start = i;
//             end = i + 1;
//             break;
//         }
//     }

//     let mut new: Vec<char> = vec![];

//     if !(start == 0 && end == 0) {
//         for c in &input[..start] {
//             new.push(*c);
//         }
//         new.push('[');
//         for c in to_char(left) {
//             new.push(c);
//         }
//         new.push(',');
//         for c in to_char(right) {
//             new.push(c);
//         }
//         new.push(']');
//         for c in &input[end + 1..] {
//             new.push(*c);
//         }
//         return (true, new);
//     }

//     (false, input.to_owned())
// }

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt"), 4140);
    }
}
