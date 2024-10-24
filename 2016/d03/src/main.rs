use std::fs;

fn main() {
    println!("Part one: {}", solve("input.txt", false));
    println!("Part two: {}", solve("input.txt", true));
}

fn solve(file: &str, two: bool) -> usize {
    let input = fs::read_to_string(file).unwrap();
    if !two {
        let valid_triangles = input
            .lines()
            .map(Triangle::from)
            .filter(|x| x.is_valid())
            .count();

        valid_triangles
    } else {
        let valid_triangles = Triangle::from_input(&input)
            .iter()
            .filter(|x| x.is_valid())
            .count();

        valid_triangles
    }
}

#[derive(Debug)]
struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}

impl Triangle {
    fn from(line: &str) -> Triangle {
        let mut sides = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        Triangle {
            a: sides.next().unwrap(),
            b: sides.next().unwrap(),
            c: sides.next().unwrap(),
        }
    }

    fn from_input(input: &str) -> Vec<Triangle> {
        let mut triangles: Vec<Triangle> = vec![];

        let lines = input.lines().collect::<Vec<&str>>();
        for i in (0..lines.len()).step_by(3) {
            let numbers1: Vec<i32> = lines[i]
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            let numbers2: Vec<i32> = lines[i + 1]
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            let numbers3: Vec<i32> = lines[i + 2]
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            triangles.push(Triangle {
                a: numbers1[0],
                b: numbers2[0],
                c: numbers3[0],
            });
            triangles.push(Triangle {
                a: numbers1[1],
                b: numbers2[1],
                c: numbers3[1],
            });
            triangles.push(Triangle {
                a: numbers1[2],
                b: numbers2[2],
                c: numbers3[2],
            });
        }

        triangles
    }

    fn is_valid(&self) -> bool {
        self.a + self.b > self.c && self.a + self.c > self.b && self.b + self.c > self.a
    }
}
