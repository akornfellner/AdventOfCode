use std::{collections::HashMap, fs};

pub fn solve(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();

    let binary: Vec<String> = input.chars().map(|x| bin(x)).collect();

    let mut tmp = String::new();

    for b in &binary {
        tmp += b;
    }

    let binary = tmp;

    check_packet(&binary);

    5
}

fn check_packet(binary: &str) {
    let chars: Vec<char> = binary.chars().collect();
    let mut values: Vec<usize> = vec![];

    let version = to_dec(&binary[0..3]);
    let typ = to_dec(&binary[3..6]);

    if typ == 4 {
        let mut i = 0usize;

        loop {
            let value = to_dec(&binary[i + 7..i + 11]);
            values.push(value);
            if chars[i + 6] == '0' {
                break;
            }
            i += 5;
        }
    } else {
        let i = chars[6];

        let mut packages = 0usize;
        let mut size = 0usize;
        let mut start = 0usize;

        if i == '0' {
            size = to_dec(&binary[7..22]);
            
        } else{
            packages = 
        }
    }

    println!("{:?}", values);
}

fn bin(hex: char) -> String {
    let b = HashMap::from([
        ('0', "0000".to_string()),
        ('1', "0001".to_string()),
        ('2', "0010".to_string()),
        ('3', "0011".to_string()),
        ('4', "0100".to_string()),
        ('5', "0101".to_string()),
        ('6', "0110".to_string()),
        ('7', "0111".to_string()),
        ('8', "1000".to_string()),
        ('9', "1001".to_string()),
        ('A', "1010".to_string()),
        ('B', "1011".to_string()),
        ('C', "1100".to_string()),
        ('D', "1101".to_string()),
        ('E', "1110".to_string()),
        ('F', "1111".to_string()),
    ]);

    b[&hex].clone()
}

fn to_dec(bin: &str) -> usize {
    let mut mul = 1;
    let mut bin: Vec<char> = bin.chars().collect();
    bin.reverse();

    let mut result = 0;

    for b in bin {
        let value = b as usize - 48;
        result += value * mul;
        mul *= 2;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt"), 31);
    }
}
