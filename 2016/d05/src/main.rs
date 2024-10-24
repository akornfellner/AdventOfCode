fn main() {
    let (p1, p2) = solve("abbhdwsy");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(input: &str) -> (String, String) {
    let mut i = 0;
    let mut password1 = String::new();
    let mut password2 = ['_'; 8];
    let mut count = (0usize, 0usize);

    loop {
        let guess = String::from(input) + &i.to_string();
        let digest = md5::compute(guess);
        let hash = format!("{:x}", digest);

        if hash.starts_with("00000") {
            if count.0 < 8 {
                password1.push(hash.chars().nth(5).unwrap());
                count.0 += 1;
            }

            let pos = hash.chars().nth(5).unwrap().to_digit(10);

            if pos.is_some() && pos.unwrap() < 8 && password2[pos.unwrap() as usize] == '_' {
                password2[pos.unwrap() as usize] = hash.chars().nth(6).unwrap();
                count.1 += 1;
            }

            if count == (8, 8) {
                break;
            }
        }

        i += 1;
    }

    (password1, password2.iter().collect())
}
