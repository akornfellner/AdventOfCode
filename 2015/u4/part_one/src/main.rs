fn main() {
    let mut i = 1usize;

    let key = String::from("ckczppom");

    loop {
        let s = i.to_string();
        let s = key.clone() + &s;
        let hash = get_hash(&s);

        let first_five: Vec<char> = hash.chars().collect();

        let mut five_zeros = true;

        for i in &first_five[..5] {
            match i {
                '0' => {}
                _ => five_zeros = false,
            }
        }

        if five_zeros {
            break;
        }

        i += 1;
    }

    println!("{}", i);
}

fn get_hash(input: &str) -> String {
    let digest = md5::compute(input.as_bytes());
    format!("{:X}", digest)
}
