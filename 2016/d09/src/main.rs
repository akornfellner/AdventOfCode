fn main() {
    println!("Part one: {}", solve("input.txt"));
    println!("Part two: {}", solve2("input.txt"));
}

fn solve(file: &str) -> usize {
    let input = std::fs::read_to_string(file).unwrap();
    let chars = input.chars().collect::<Vec<char>>();
    let mut pos = 0;
    let mut count = 0;
    let mut save = 0;

    while pos < chars.len() {
        if chars[pos] == '(' {
            save = pos + 1;
            pos += 1;
        } else if chars[pos] == ')' {
            let sub = chars[save..pos].iter().collect::<String>();
            let mut parts = sub.split('x');
            let len = parts.next().unwrap().parse::<usize>().unwrap();
            let rep = parts.next().unwrap().parse::<usize>().unwrap();
            count += len * rep - sub.len();
            pos += len + 1;
        } else {
            count += 1;
            pos += 1;
        }
    }

    count
}

fn solve2(file: &str) -> usize {
    let input = std::fs::read_to_string(file).unwrap();
    let chars = input.chars().collect::<Vec<char>>();
    let mut count = 0;
    let mut save = 0;
    let mut mult: Vec<(usize, usize)> = vec![];
    let mut compressor = false;

    for (i, c) in chars.iter().enumerate() {
        let c = *c;
        if c == '(' {
            save = i + 1;
            compressor = true;
        } else if c == ')' {
            let sub = chars[save..i].iter().collect::<String>();
            let mut parts = sub.split('x');
            let len = parts.next().unwrap().parse::<usize>().unwrap();
            let rep = parts.next().unwrap().parse::<usize>().unwrap();
            mult.push((len, rep));
            compressor = false;
        } else if !compressor {
            let mut add = 1;
            for (_, rep) in mult.iter() {
                add *= rep;
            }
            count += add;
        }

        let mut new_mult = vec![];
        for (len, rep) in mult.iter() {
            if *len > 0 {
                new_mult.push((*len - 1, *rep));
            }
        }
        mult = new_mult;
    }

    count
}
