use std::collections::HashMap;

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {p1}");
    println!("Part two: {p2}");
}

fn solve(file: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(file).unwrap();
    let mut one = 0;

    let mut bots: HashMap<usize, Bot> = HashMap::new();
    let mut outputs: HashMap<usize, Vec<usize>> = HashMap::new();

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words[0] == "value" {
            let chip = words[1].parse::<usize>().unwrap();
            let bot = words[5].parse::<usize>().unwrap();
            bots.entry(bot).or_insert_with(Bot::new);
            bots.get_mut(&bot).unwrap().add_chip(chip);
        } else {
            let bot = words[1].parse::<usize>().unwrap();
            let low_dest = match words[5] {
                "bot" => Destination::Bot(words[6].parse::<usize>().unwrap()),
                "output" => Destination::Output(words[6].parse::<usize>().unwrap()),
                _ => panic!("Invalid destination"),
            };
            let high_dest = match words[10] {
                "bot" => Destination::Bot(words[11].parse::<usize>().unwrap()),
                "output" => Destination::Output(words[11].parse::<usize>().unwrap()),
                _ => panic!("Invalid destination"),
            };
            bots.entry(bot).or_insert_with(Bot::new);
            bots.get_mut(&bot)
                .unwrap()
                .tasks
                .push((low_dest, high_dest));
        }
    }

    let mut done = false;

    while !done {
        let bot = bots.iter().find(|(_, bot)| bot.is_full());

        match bot {
            Some((id, bot)) => {
                let mut bot = bot.clone();
                let low = bot.chips[0];
                let high = bot.chips[1];
                if low == 17 && high == 61 {
                    one = *id;
                }
                bot.chips.clear();
                let (l, h) = bot.tasks.pop().unwrap();
                bots.insert(*id, bot);
                match l {
                    Destination::Bot(b) => {
                        bots.get_mut(&b).unwrap().add_chip(low);
                    }
                    Destination::Output(o) => {
                        outputs.entry(o).or_default();
                        outputs.get_mut(&o).unwrap().push(low);
                    }
                }
                match h {
                    Destination::Bot(b) => {
                        bots.get_mut(&b).unwrap().add_chip(high);
                    }
                    Destination::Output(o) => {
                        outputs.entry(o).or_default();
                        outputs.get_mut(&o).unwrap().push(high);
                    }
                }
            }
            None => done = true,
        }
    }

    (one, outputs[&0][0] * outputs[&1][0] * outputs[&2][0])
}

#[derive(Debug, Clone)]
struct Bot {
    chips: Vec<usize>,
    tasks: Vec<(Destination, Destination)>,
}

impl Bot {
    fn new() -> Bot {
        Bot {
            chips: vec![],
            tasks: vec![],
        }
    }

    fn add_chip(&mut self, chip: usize) {
        if self.chips.len() < 2 {
            self.chips.push(chip);
        } else {
            panic!("Bot already has two chips");
        }
        self.chips.sort();
    }

    fn is_full(&self) -> bool {
        self.chips.len() == 2
    }
}

#[derive(Debug, Clone, Copy)]
enum Destination {
    Bot(usize),
    Output(usize),
}
