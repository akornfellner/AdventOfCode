use std::fs;

pub fn solve(input: &str) -> usize {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut sues: Vec<Sue> = vec![];

    for line in lines {
        sues.push(Sue::from(line));
    }

    let real = Sue {
        number: 0,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    let mut result = 0;

    for sue in sues {
        if sue == real {
            result = sue.number;
        }
    }

    result
}

#[derive(Debug)]
struct Sue {
    number: usize,

    children: Option<i32>,
    cats: Option<i32>,
    samoyeds: Option<i32>,
    pomeranians: Option<i32>,
    akitas: Option<i32>,
    vizslas: Option<i32>,
    goldfish: Option<i32>,
    trees: Option<i32>,
    cars: Option<i32>,
    perfumes: Option<i32>,
}

impl Sue {
    fn from(input: &str) -> Self {
        let mut sue = Self {
            number: 0,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        };

        let input = input.replace(':', "").replace(',', "");
        let parts: Vec<&str> = input.split(' ').collect();

        sue.number = parts[1].parse::<usize>().unwrap();

        let keys = [
            parts[2].to_string(),
            parts[4].to_string(),
            parts[6].to_string(),
        ];

        let values = [
            parts[3].parse::<i32>().unwrap(),
            parts[5].parse::<i32>().unwrap(),
            parts[7].parse::<i32>().unwrap(),
        ];

        for i in 0..keys.len() {
            match keys[i].as_str() {
                "children" => sue.children = Some(values[i]),
                "cats" => sue.cats = Some(values[i]),
                "samoyeds" => sue.samoyeds = Some(values[i]),
                "pomeranians" => sue.pomeranians = Some(values[i]),
                "akitas" => sue.akitas = Some(values[i]),
                "vizslas" => sue.vizslas = Some(values[i]),
                "goldfish" => sue.goldfish = Some(values[i]),
                "trees" => sue.trees = Some(values[i]),
                "cars" => sue.cars = Some(values[i]),
                "perfumes" => sue.perfumes = Some(values[i]),
                _ => {}
            }
        }

        sue
    }
}

impl PartialEq for Sue {
    fn eq(&self, other: &Self) -> bool {
        if let Some(first) = self.children {
            if let Some(second) = other.children {
                if first != second {
                    return false;
                }
            }
        }

        if let Some(first) = self.cats {
            if let Some(second) = other.cats {
                if first <= second {
                    return false;
                }
            }
        }

        if let Some(first) = self.samoyeds {
            if let Some(second) = other.samoyeds {
                if first != second {
                    return false;
                }
            }
        }

        if let Some(first) = self.pomeranians {
            if let Some(second) = other.pomeranians {
                if first >= second {
                    return false;
                }
            }
        }

        if let Some(first) = self.akitas {
            if let Some(second) = other.akitas {
                if first != second {
                    return false;
                }
            }
        }

        if let Some(first) = self.vizslas {
            if let Some(second) = other.vizslas {
                if first != second {
                    return false;
                }
            }
        }

        if let Some(first) = self.goldfish {
            if let Some(second) = other.goldfish {
                if first >= second {
                    return false;
                }
            }
        }

        if let Some(first) = self.trees {
            if let Some(second) = other.trees {
                if first <= second {
                    return false;
                }
            }
        }

        if let Some(first) = self.cars {
            if let Some(second) = other.cars {
                if first != second {
                    return false;
                }
            }
        }

        if let Some(first) = self.perfumes {
            if let Some(second) = other.perfumes {
                if first != second {
                    return false;
                }
            }
        }

        true
    }
}
