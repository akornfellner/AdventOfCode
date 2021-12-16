use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let binary: Vec<String> = input.chars().map(|x| bin(x)).collect();

    let mut tmp = String::new();

    for b in &binary {
        tmp += b;
    }

    let binary = tmp;

    check_packet(&binary).1
}

fn check_packet(binary: &str) -> (usize, usize) {
    let chars: Vec<char> = binary.chars().collect();
    let mut values: Vec<usize> = vec![];

    let typ = to_dec(&binary[3..6]);

    let mut length;

    if typ == 4 {
        let mut i = 0usize;
        let mut value = String::new();

        loop {
            value += &binary[i + 7..i + 11];
            if chars[i + 6] == '0' {
                break;
            }
            i += 5;
        }
        return (11 + i, to_dec(&value));
    } else {
        let i = chars[6];

        if i == '0' {
            let size = to_dec(&binary[7..22]);
            let mut start = 22usize;
            length = 22;

            while start < size + 22 {
                let (l, v) = check_packet(&binary[start..]);
                values.push(v);
                start += l;
                length += l;
            }
        } else {
            let packets = to_dec(&binary[7..18]);
            let mut start = 18usize;
            length = 18;

            for _ in 0..packets {
                let (l, v) = check_packet(&binary[start..]);
                values.push(v);
                start += l;
                length += l;
            }
        }
    }

    let result: usize = match typ {
        0 => values.into_iter().sum(),
        1 => values.into_iter().product(),
        2 => values.into_iter().min().unwrap(),
        3 => values.into_iter().max().unwrap(),
        5 => (values[0] > values[1]) as usize,
        6 => (values[0] < values[1]) as usize,
        7 => (values[0] == values[1]) as usize,
        _ => 0,
    };

    (length, result)
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
    fn test_sum() {
        assert_eq!(solve("C200B40A82"), 3);
    }

    #[test]
    fn test_product() {
        assert_eq!(solve("04005AC33890"), 54);
    }

    #[test]
    fn test_min() {
        assert_eq!(solve("880086C3E88112"), 7);
    }

    #[test]
    fn test_max() {
        assert_eq!(solve("CE00C43D881120"), 9);
    }

    #[test]
    fn test_less() {
        assert_eq!(solve("D8005AC2A8F0"), 1);
    }

    #[test]
    fn test_equal() {
        assert_eq!(solve("9C005AC2F8F0"), 0);
    }

    #[test]
    fn test_combine() {
        assert_eq!(solve("9C0141080250320F1802104A08"), 1);
    }
}
