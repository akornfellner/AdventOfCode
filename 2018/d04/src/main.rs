use entry::*;
use std::{collections::HashMap, env::args};
mod entry;

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

    let mut entries: Vec<Entry> = input.lines().map(Entry::new).collect();

    entries.sort();

    let mut guards: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    let mut curr_id: usize = 0;
    let mut curr_start: usize = 0;

    for entry in &entries {
        match entry.entry_type {
            EntryType::Begin(guard_id) => {
                curr_id = guard_id;
            }
            EntryType::FallAsleep => {
                curr_start = entry.minute;
            }
            EntryType::WakeUp => {
                guards
                    .entry(curr_id)
                    .or_default()
                    .push((curr_start, entry.minute));
            }
        }
    }

    let (max_id, _) = guards
        .iter()
        .map(|(id, times)| {
            let total_minutes: usize = times.iter().map(|(start, end)| end - start).sum();
            (*id, total_minutes)
        })
        .max_by_key(|(_, total)| *total)
        .unwrap_or((0, 0));

    let max_minutes = get_max_minute(&guards);

    let p1 = max_minutes.get(&max_id).unwrap().0 * max_id;

    let (p2_id, p2_minute) = max_minutes
        .iter()
        .max_by_key(|(_, (_, count))| *count)
        .map(|(&id, &(minute, _))| (id, minute))
        .unwrap_or((0, 0));

    let p2 = p2_id * p2_minute;

    (p1, p2)
}

fn get_max_minute(guards: &HashMap<usize, Vec<(usize, usize)>>) -> HashMap<usize, (usize, usize)> {
    let mut max_minutes: HashMap<usize, (usize, usize)> = HashMap::new();

    for guard in guards.keys() {
        let mut minutes: HashMap<usize, usize> = HashMap::new();

        for minute in 0..60 {
            let entry = minutes.entry(minute).or_insert(0);
            for (start, end) in guards.get(guard).unwrap() {
                if minute >= *start && minute < *end {
                    *entry += 1;
                }
            }
        }

        let max_minute: (usize, usize) = minutes
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(&minute, &count)| (minute, count))
            .unwrap_or((0, 0));

        max_minutes.insert(*guard, max_minute);
    }

    max_minutes
}
