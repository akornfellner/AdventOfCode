pub fn solve(input: &str, rounds: usize) -> usize {
    let mut result = input.to_string();

    for _ in 0..rounds {
        round(&mut result);
    }

    result.len()
}

fn round(input: &mut String) {
    let old = input.to_string();
    input.clear();

    let chars: Vec<char> = old.chars().collect();

    let mut cur = chars[0];
    let mut count = 1usize;

    for i in 1..old.len() {
        if chars[i] == cur {
            count += 1;
        } else {
            input.push_str(&count.to_string());
            input.push(cur);

            count = 1;
            cur = chars[i];
        }
    }

    input.push_str(&count.to_string());
    input.push(cur);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve("1", 5);
        assert_eq!(result, 6);
    }
}
