use std::{cmp::Ordering, collections::HashMap};
fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    let mut modules: Mod = HashMap::new();

    for line in input.lines() {
        let parts = line.split(" -> ").collect::<Vec<&str>>();

        let dests = parts[1]
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let (typ, start) = match parts[0].chars().next().unwrap() {
            '&' => (Module::Conjuction(dests, HashMap::new()), 1),
            '%' => (Module::FlipFlop(false, dests), 1),
            _ => (Module::Broadcast(dests), 0),
        };

        modules.insert(parts[0][start..].to_string(), typ);
    }

    for modul in &modules.clone() {
        let dests = match modul.1 {
            Module::Conjuction(d, _) => d,
            Module::FlipFlop(_, d) => d,
            Module::Broadcast(d) => d,
        };

        for dest in dests {
            if let Some(Module::Conjuction(_, inputs)) = modules.get_mut(dest) {
                inputs.insert(modul.0.clone(), Pulse::Low);
            }
        }
    }

    let mut inputs: HashMap<String, usize> = HashMap::new();

    for module in modules.values() {
        if let Module::Conjuction(dests, ins) = module {
            if dests.contains(&"rx".to_string()) {
                inputs = ins
                    .keys()
                    .map(|s| (s.clone(), 0))
                    .collect::<HashMap<String, usize>>();
                break;
            }
        }
    }

    let mut count = 0;
    let mut sum = (0, 0);

    result.0 = sum.0 * sum.1;

    'outer: loop {
        count += 1;
        sum.0 += 1;

        let mut queue: Vec<(String, Pulse)> = vec![("broadcaster".to_string(), Pulse::Low)];
        while !queue.is_empty() {
            let mut new_queue: Vec<(String, Pulse)> = Vec::new();
            for (name, pulse) in queue {
                if let Pulse::Low = pulse {
                    if inputs.contains_key(&name) {
                        let entry = inputs.get_mut(&name).unwrap();
                        if *entry == 0 {
                            *entry = count;
                        }
                    }
                }
                if modules.get(&name).is_none() {
                    continue;
                }
                add_pulses(&mut sum, send(&name, pulse, &mut modules, &mut new_queue));
            }
            queue = new_queue;
        }

        match count.cmp(&1000) {
            Ordering::Less => continue,
            Ordering::Equal => result.0 = sum.0 * sum.1,
            Ordering::Greater => (),
        }

        for (_, value) in inputs.clone() {
            if value == 0 {
                continue 'outer;
            }
        }
        break;
    }

    result.1 = inputs.values().product();

    result
}

type Mod = HashMap<String, Module>;

#[derive(Debug, Clone)]
enum Module {
    Conjuction(Vec<String>, HashMap<String, Pulse>),
    FlipFlop(bool, Vec<String>),
    Broadcast(Vec<String>),
}

fn send(
    name: &str,
    pulse: Pulse,
    modules: &mut Mod,
    queue: &mut Vec<(String, Pulse)>,
) -> (usize, usize) {
    let mut result = (0, 0);
    let module = modules.get_mut(name).unwrap();

    match module {
        Module::Conjuction(dests, inputs) => {
            let mut all_high = true;
            for input in inputs.values() {
                if let Pulse::Low = input {
                    all_high = false;
                    break;
                }
            }
            if all_high {
                result.0 += dests.len();
                for dest in &dests.clone() {
                    queue.push((dest.clone(), Pulse::Low));
                    update_inputs(dest, name, Pulse::Low, modules);
                }
            } else {
                result.1 += dests.len();
                for dest in &dests.clone() {
                    queue.push((dest.clone(), Pulse::High));
                    update_inputs(dest, name, Pulse::High, modules);
                }
            }
        }
        Module::FlipFlop(state, dests) => {
            if let Pulse::Low = pulse {
                if *state {
                    *state = false;
                    result.0 += dests.len();
                    for dest in &dests.clone() {
                        queue.push((dest.clone(), Pulse::Low));
                        update_inputs(dest, name, Pulse::Low, modules);
                    }
                } else {
                    *state = true;
                    result.1 += dests.len();
                    for dest in &dests.clone() {
                        queue.push((dest.clone(), Pulse::High));
                        update_inputs(dest, name, Pulse::High, modules);
                    }
                }
            }
        }
        Module::Broadcast(dests) => {
            match pulse {
                Pulse::Low => result.0 += dests.len(),
                Pulse::High => result.1 += dests.len(),
            }
            for dest in &dests.clone() {
                queue.push((dest.clone(), pulse));
                update_inputs(dest, name, pulse, modules);
            }
        }
    }

    result
}

fn add_pulses(current: &mut (usize, usize), new: (usize, usize)) {
    current.0 += new.0;
    current.1 += new.1;
}

fn update_inputs(dest: &str, name: &str, pulse: Pulse, modules: &mut Mod) {
    if let Some(Module::Conjuction(_, inputs)) = modules.get_mut(dest) {
        let entry = inputs.entry(name.to_string()).or_insert(Pulse::Low);
        *entry = pulse;
    }
}

#[derive(Debug, Clone, Copy)]
enum Pulse {
    Low,
    High,
}
