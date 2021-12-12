use std::vec;

#[derive(Debug)]
pub struct Bag {
    pub name: String,
    pub childs: Vec<(String, usize)>,
}

impl Bag {
    pub fn new(name: &str, childs: Vec<(String, usize)>) -> Self {
        Bag {
            name: String::from(name),
            childs,
        }
    }

    pub fn new_from_line(input: &str) -> Self {
        let l: Vec<&str> = input.split(" bags").collect();
        let name = l[0];

        let mut tmp: Vec<&str> = vec![];

        let numbers = [
            " 1 ", " 2 ", " 3 ", " 4 ", " 5 ", " 6 ", " 7 ", " 8 ", " 9 ",
        ];

        for i in l {
            let a: Vec<&str> = i.split(" bag").collect();
            for j in a {
                tmp.push(j);
            }
        }

        let mut childs: Vec<(String, usize)> = vec![];

        for i in tmp {
            let mut position = 1usize;
            for n in numbers {
                let a: Vec<&str> = i.split(n).collect();
                if a.len() > 1 {
                    childs.push((String::from(a[1]), position));
                }
                position += 1;
            }
        }

        Bag::new(name, childs)
    }

    pub fn get_bag(name: &str, bags: &[Bag]) -> Vec<(String, usize)> {
        for i in bags {
            if i.name == name {
                return i.childs.clone();
            }
        }
        vec![]
    }
}
