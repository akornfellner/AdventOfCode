use std::{collections::HashMap, env::args};

fn main() {
    let filename = args().nth(1).unwrap_or("input_test.txt".to_string());
    let (p1, p2) = solve(&filename);
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();

    let mut counter: HashMap<(usize, usize), usize> = HashMap::new();
    let rectangles: Vec<Rectangle> = input.lines().map(Rectangle::new).collect();

    for rectangle in &rectangles {
        let fields = rectangle.get_fields();
        for field in fields {
            *counter.entry(field).or_insert(0) += 1;
        }
    }

    let p1 = counter.values().filter(|&&v| v > 1).count();
    let mut p2 = 0;

    for rectangle in &rectangles {
        let fields = rectangle.get_fields();
        if fields.iter().all(|&field| counter[&field] == 1) {
            p2 = rectangle.id;
            break;
        }
    }

    (p1, p2)
}

#[derive(Debug)]
struct Rectangle {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Rectangle {
    fn new(line: &str) -> Self {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let id = parts[0][1..].parse::<usize>().unwrap();
        let coords = parts[2]
            .trim_end_matches(':')
            .split(',')
            .collect::<Vec<&str>>();
        let x = coords[0].parse::<usize>().unwrap();
        let y = coords[1].parse::<usize>().unwrap();
        let dimensions = parts[3].split('x').collect::<Vec<&str>>();
        let width = dimensions[0].parse::<usize>().unwrap();
        let height = dimensions[1].parse::<usize>().unwrap();
        Rectangle {
            id,
            x,
            y,
            width,
            height,
        }
    }

    fn get_fields(&self) -> Vec<(usize, usize)> {
        let mut fields = vec![];
        for i in self.x..(self.x + self.width) {
            for j in self.y..(self.y + self.height) {
                fields.push((i, j));
            }
        }
        fields
    }
}
