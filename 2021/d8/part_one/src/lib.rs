use std::fs;

pub fn solve(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).unwrap();

    let lines: Vec<&str> = input.split("\n").collect();

    let digitlist: Vec<&str> = lines
        .into_iter()
        .map(|x| x.split(" | ").collect::<Vec<&str>>()[1])
        .collect();

    let mut count = 0;

    for d in digitlist {
        let single: Vec<&str> = d.split(' ').collect();
        for i in single {
            match i.len() {
                2 | 4 | 3 | 7 => count += 1,
                _ => (),
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt"), 26);
    }
}
