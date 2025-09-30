use std::env::args;

type Point = (usize, usize);

fn main() {
    let filename = args().nth(1).unwrap_or("input_test.txt".to_string());
    let (part_one, part_two) = solve(&filename);
    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();

    let (points, max_x, max_y) = parse_points(&input);
    let grid = build_voronoi_grid(&points, max_x, max_y);

    let part_one = find_largest_finite_area(&grid, &points, max_x, max_y);
    let part_two = count_safe_region(&points, max_x, max_y);

    (part_one, part_two)
}

fn parse_points(input: &str) -> (Vec<Point>, usize, usize) {
    let mut points = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(", ").collect();
        let x = parts[0].parse::<usize>().unwrap();
        let y = parts[1].parse::<usize>().unwrap();

        points.push((x, y));
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    (points, max_x, max_y)
}

fn build_voronoi_grid(points: &[Point], max_x: usize, max_y: usize) -> Vec<Vec<isize>> {
    let mut grid = vec![vec![-1isize; max_x + 1]; max_y + 1];

    for y in 0..=max_y {
        for x in 0..=max_x {
            let current_point = (x, y);
            let mut min_distance = usize::MAX;
            let mut closest_index = -1isize;
            let mut has_tie = false;

            for (index, point) in points.iter().enumerate() {
                let distance = manhattan_distance(&current_point, point);

                if distance < min_distance {
                    min_distance = distance;
                    closest_index = index as isize;
                    has_tie = false;
                } else if distance == min_distance {
                    has_tie = true;
                }
            }

            grid[y][x] = if has_tie { -1 } else { closest_index };
        }
    }

    grid
}

fn find_infinite_indices(grid: &[Vec<isize>], max_x: usize, max_y: usize) -> Vec<isize> {
    let mut infinite_indices = Vec::new();

    for x in 0..=max_x {
        add_if_not_exists(&mut infinite_indices, grid[0][x]);
        add_if_not_exists(&mut infinite_indices, grid[max_y][x]);
    }

    for y in 0..=max_y {
        add_if_not_exists(&mut infinite_indices, grid[y][0]);
        add_if_not_exists(&mut infinite_indices, grid[y][max_x]);
    }

    infinite_indices
}

fn add_if_not_exists(vec: &mut Vec<isize>, index: isize) {
    if index != -1 && !vec.contains(&index) {
        vec.push(index);
    }
}

fn find_largest_finite_area(
    grid: &[Vec<isize>],
    points: &[Point],
    max_x: usize,
    max_y: usize,
) -> usize {
    let infinite_indices = find_infinite_indices(grid, max_x, max_y);
    let mut area_counts = vec![0usize; points.len()];

    for row in grid {
        for &cell in row {
            if cell != -1 && !infinite_indices.contains(&cell) {
                area_counts[cell as usize] += 1;
            }
        }
    }

    *area_counts.iter().max().unwrap()
}

fn count_safe_region(points: &[Point], max_x: usize, max_y: usize) -> usize {
    let mut safe_count = 0;

    for x in 0..=max_x {
        for y in 0..=max_y {
            let total_distance: usize = points
                .iter()
                .map(|point| manhattan_distance(&(x, y), point))
                .sum();

            if total_distance < 10000 {
                safe_count += 1;
            }
        }
    }

    safe_count
}

fn manhattan_distance(a: &Point, b: &Point) -> usize {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
}
