pub fn solve(input: &str) -> usize {
    let binary: Vec<String> = input.chars().map(bin).collect();

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

    let version = to_dec(&binary[0..3]);
    let typ = to_dec(&binary[3..6]);

    let mut result = version;
    let mut length;

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
        length = 11 + i;
    } else {
        let i = chars[6];

        if i == '0' {
            let size = to_dec(&binary[7..22]);
            let mut start = 22usize;
            length = 22;

            while start < size + 22 {
                let (l, v) = check_packet(&binary[start..]);
                result += v;
                start += l;
                length += l;
            }
        } else {
            let packets = to_dec(&binary[7..18]);
            let mut start = 18usize;
            length = 18;

            for _ in 0..packets {
                let (l, v) = check_packet(&binary[start..]);
                result += v;
                start += l;
                length += l;
            }
        }
    }

    (length, result)
}

fn bin(hex: char) -> String {
    let h = match hex {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    };
    h.to_string()
}

fn to_dec(bin: &str) -> usize {
    usize::from_str_radix(bin, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        assert_eq!(solve("8A004A801A8002F478"), 16);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("620080001611562C8802118E34"), 12);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("C0015000016115A2E0802F182340"), 23);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve("A0016C880162017C3686B18A3D4780"), 31);
    }
}
