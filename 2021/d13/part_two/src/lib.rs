use std::fs;

pub fn solve(filename: &str) -> String {
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

    for fold in &folds {
        for dot in &mut dots {
            if fold.axes == "x" && dot.x > fold.value {
                dot.x = 2 * fold.value - dot.x;
            }

            if fold.axes == "y" && dot.y > fold.value {
                dot.y = 2 * fold.value - dot.y;
            }
        }
    }

    let mut tmp: Vec<Point> = vec![];

    for dot in &dots {
        if !tmp.contains(dot) {
            tmp.push(Point { x: dot.x, y: dot.y })
        }
    }

    let dots = tmp;

    print_field(&dots)
}

fn print_field(dots: &[Point]) -> String {
    let mut result = String::new();

    let mut max_x = 0usize;
    let mut max_y = 0usize;

    for dot in dots {
        if dot.x > max_x {
            max_x = dot.x;
        }
        if dot.y > max_y {
            max_y = dot.y;
        }
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            if dots.contains(&Point { x, y }) {
                result += "*";
            } else {
                result += " ";
            }
        }
        result += "\n";
    }

    result
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
        assert_eq!(
            solve("input_test.txt"),
            "*****\n*   *\n*   *\n*   *\n*****\n"
        );
    }
}
