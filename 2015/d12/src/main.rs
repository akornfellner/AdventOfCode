use d12::solve;

fn main() {
    println!("part one: {}", solve("input.txt", false).unwrap());
    println!("part two: {}", solve("input.txt", true).unwrap());
}
