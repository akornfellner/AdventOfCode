fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (i32, i32) {
    let input = std::fs::read_to_string(filename).unwrap();
    let records: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut invalid_records: Vec<Vec<i32>> = vec![];

    let mut p1 = 0;

    for record in &records {
        if check_record(record) {
            p1 += 1;
        } else {
            invalid_records.push(record.clone());
        }
    }

    let mut p2 = p1;

    for record in &invalid_records {
        for r in 0..record.len() {
            let mut new_record = record.clone();
            new_record.remove(r);
            if check_record(&new_record) {
                p2 += 1;
                break;
            }
        }
    }

    (p1, p2)
}

fn compare(first: i32, second: i32, inc: bool) -> bool {
    let diff = second - first;
    inc && diff > 0 && diff < 4 || !inc && diff < 0 && diff > -4
}

fn check_record(record: &[i32]) -> bool {
    let inc = record[0] < record[1];
    for i in 1..record.len() {
        if !compare(record[i - 1], record[i], inc) {
            return false;
        }
    }
    true
}
