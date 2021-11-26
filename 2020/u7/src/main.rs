use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Reading errors");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut bags: Vec<Bag> = vec![];

    for i in lines {
        bags.push(Bag::new_from_string(i));
    }
}

fn search_bag(name: &str, bags: Vec<Bag>) -> bool {
    let mut bag: Bag;
    for i in bags {
        if i.name == String::from(name) {
            bag = i;
            break;
        }
    }
    let mut found = false;
    found
}

#[derive(Debug)]
struct Bag {
    name: String,
    bags: Vec<String>,
}

impl Bag {
    fn new(name: &str, bags: Vec<String>) -> Self {
        Bag {
            name: String::from(name),
            bags: bags,
        }
    }

    fn new_from_string(input: &str) -> Self {
        let s: Vec<&str> = input.split(" bags").collect();
        let name = s[0];

        let mut k: Vec<&str> = vec![];

        for i in s {
            let t: Vec<&str> = i.split(" bag").collect();
            for j in t {
                k.push(j)
            }
        }

        let names = &k[1..];

        let numbers = [
            " 1 ", " 2 ", " 3 ", " 4 ", " 5 ", " 6 ", " 7 ", " 8 ", " 9 ",
        ];

        let mut arr: Vec<&str> = vec![];

        for number in numbers {
            for i in names {
                let v = *i;
                let values: Vec<&str> = v.split(number).collect();
                if values.len() == 2 {
                    arr.push(values[1])
                }
            }
        }

        let mut bags: Vec<String> = vec![];

        for i in arr {
            bags.push(String::from(i));
        }

        let bag = Bag::new(name, bags);
        bag
    }
}
