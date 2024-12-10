use stopwatch::time;

#[time]
fn main() {
    let (p1, p2) = solve("input_test.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();

    (0, 0)
}
