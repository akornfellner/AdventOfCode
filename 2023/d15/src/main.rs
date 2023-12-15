fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    result.0 += input.split(',').map(hash).sum::<usize>();

    let mut boxes: Vec<Vec<Box>> = vec![vec![]; 256];

    for code in input.split(',') {
        if code.contains('=') {
            let parts = code.split('=').collect::<Vec<&str>>();
            let label = parts[0].trim();
            let hash = hash(label);
            match boxes[hash].iter().position(|b| b.label == label) {
                Some(index) => {
                    boxes[hash][index].value = parts[1].trim().parse().unwrap();
                }
                None => {
                    boxes[hash].push(Box::new(
                        label.to_string(),
                        parts[1].trim().parse().unwrap(),
                    ));
                }
            }
        } else {
            let label = code.split('-').collect::<Vec<&str>>()[0].trim();
            let hash = hash(label);
            if let Some(index) = boxes[hash].iter().position(|b| b.label == label) {
                boxes[hash].remove(index);
            }
        }
    }

    for (i, b) in boxes.iter().enumerate() {
        for (j, focal) in b.iter().enumerate() {
            result.1 += (i + 1) * (j + 1) * focal.value;
        }
    }

    result
}

fn hash(value: &str) -> usize {
    let mut result = 0;

    for c in value.chars() {
        result += c as usize;
        result = result * 17 % 256
    }

    result
}

#[derive(Debug, Clone)]
struct Box {
    label: String,
    value: usize,
}

impl Box {
    fn new(label: String, value: usize) -> Self {
        Self { label, value }
    }
}
