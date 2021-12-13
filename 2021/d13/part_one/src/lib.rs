use std::fs;

pub fn solve(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();

    let parts: Vec<&str> = input.split("\n\n").collect();

    let dots: Vec<&str> = parts[0].lines().collect();
    let folds: Vec<&str> = parts[1].lines().collect();

    let mut tmp: Vec<Point> = vec![];

    for dot in dots {
        let dot: Vec<usize> = dot
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        tmp.push(Point {
            x: dot[0],
            y: dot[1],
        });
    }

    let mut dots = tmp;

    let mut tmp: Vec<Fold> = vec![];

    for fold in folds {
        let fold = fold.replace("fold along ", "");
        let fold: Vec<&str> = fold.split('=').collect();
        tmp.push(Fold {
            axes: fold[0].to_string(),
            value: fold[1].parse::<usize>().unwrap(),
        });
    }

    let folds = tmp;

    let fold = folds[0].clone();

    for dot in &mut dots {
        if fold.axes == "x" && dot.x > fold.value {
            dot.x = 2 * fold.value - dot.x;
        }

        if fold.axes == "y" && dot.y > fold.value {
            dot.y = 2 * fold.value - dot.y;
        }
    }

    let mut tmp: Vec<Point> = vec![];

    for dot in &dots {
        if !tmp.contains(dot) {
            tmp.push(Point { x: dot.x, y: dot.y })
        }
    }

    let dots = tmp;

    dots.len()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Fold {
    axes: String,
    value: usize,
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt"), 17);
    }
}
