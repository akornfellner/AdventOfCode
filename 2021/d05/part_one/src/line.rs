#[derive(Debug)]
pub struct Line {
    pub points: Vec<(i32, i32)>,
}

impl Line {
    pub fn new(input: &str) -> Option<Self> {
        let points: Vec<&str> = input.split(" -> ").collect();
        let x1 = points[0]
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()[0];
        let y1 = points[0]
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()[1];
        let x2 = points[1]
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()[0];
        let y2 = points[1]
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()[1];

        if !(x1 == x2 || y1 == y2) {
            return None;
        }

        let mut points: Vec<(i32, i32)> = vec![];

        if x1 == x2 {
            let mut first = y1;
            let mut second = y2;
            if second < first {
                std::mem::swap(&mut second, &mut first);
            }
            for i in first..=second {
                points.push((x1, i))
            }
        } else {
            let mut first = x1;
            let mut second = x2;
            if second < first {
                std::mem::swap(&mut second, &mut first);
            }
            for i in first..=second {
                points.push((i, y1))
            }
        }

        Some(Line { points })
    }
}
