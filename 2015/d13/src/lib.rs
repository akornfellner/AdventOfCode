use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn solve(input: &str, two: bool) -> i32 {
    let mut persons: Persons = HashSet::new();
    let mut values = get_values(input, &mut persons);

    if two {
        add_me(&mut persons, &mut values);
    }

    let mut persons: Vec<String> = persons.into_iter().collect();

    let size = persons.len();

    let mut permutations: Vec<Vec<String>> = vec![];

    permut(&mut persons, size, &mut permutations);

    let mut result = i32::MIN;

    for permut in &permutations {
        let x = get_sum(permut, &values);

        if x > result {
            result = x;
        }
    }

    result
}

fn add_me(persons: &mut Persons, values: &mut Values) {
    for person in persons.iter() {
        values.insert((String::from("me"), person.to_owned()), 0);
        values.insert((person.to_owned(), String::from("me")), 0);
    }

    persons.insert(String::from("me"));
}

fn get_values(input: &str, persons: &mut Persons) -> Values {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut values: Values = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let first = parts[0].to_string();

        let sign = match parts[2] {
            "lose" => -1,
            _ => 1,
        };

        let value = parts[3].parse::<i32>().unwrap();

        let second = parts[10].replace('.', "");

        values.insert((first.clone(), second), value * sign);
        persons.insert(first);
    }

    values
}

fn get_sum(sitting: &Vec<String>, values: &Values) -> i32 {
    let mut sum = 0;

    for i in 0..sitting.len() {
        let mut x = i + 1;

        if i == sitting.len() - 1 {
            x = 0;
        }

        let first = sitting[i].clone();
        let second = sitting[x].clone();

        sum += values.get(&(first.clone(), second.clone())).unwrap()
            + values.get(&(second, first)).unwrap();
    }

    sum
}

fn permut(persons: &mut Vec<String>, size: usize, permutations: &mut Vec<Vec<String>>) {
    if size == 1 {
        permutations.push(persons.to_owned());
        return;
    }

    for i in 0..size {
        permut(persons, size - 1, permutations);

        if size % 2 == 1 {
            (persons[0], persons[size - 1]) = (persons[size - 1].clone(), persons[0].clone());
        } else {
            (persons[i], persons[size - 1]) = (persons[size - 1].clone(), persons[i].clone());
        }
    }
}

type Values = HashMap<(String, String), i32>;
type Persons = HashSet<String>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve("input_test.txt", false);
        assert_eq!(result, 330);
    }
}
