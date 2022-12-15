use std::fs;

pub fn solve(input: &str, two: bool) -> i32 {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut ingredients: Vec<Ingredient> = vec![];

    for line in lines {
        ingredients.push(Ingredient::from(line));
    }

    let n = ingredients.len();

    let combinations = get_combinations(n);

    let mut max = i32::MIN;

    for combi in combinations {
        let mut total = Ingredient::new();
        for i in 0..n {
            total.capacity += ingredients[i].capacity * combi[i];
            total.durability += ingredients[i].durability * combi[i];
            total.flavor += ingredients[i].flavor * combi[i];
            total.texture += ingredients[i].texture * combi[i];
            total.calories += ingredients[i].calories * combi[i];
        }

        let value = total.product();

        if value > max && (total.calories == 500 || !two) {
            max = value;
        }
    }

    max
}

fn get_combinations(n: usize) -> Vec<Vec<i32>> {
    let first = vec![0; n];
    let mut result: Vec<Vec<i32>> = vec![];

    let mut combi = next(&first);

    while combi != first {
        if sum(&combi) == 100 {
            result.push(combi.clone());
        }
        combi = next(&combi);
    }

    result
}

fn sum(combi: &[i32]) -> i32 {
    let mut sum = 0;

    for value in combi {
        sum += value;
    }

    sum
}

fn next(combi: &Vec<i32>) -> Vec<i32> {
    let mut result = combi.clone();

    let mut finished = false;
    let mut index = (combi.len() - 1) as i32;

    while !finished && index >= 0 {
        let i = index as usize;
        result[i] = (result[i] + 1) % 101;
        if result[i] == 0 {
            index -= 1;
        } else {
            finished = true;
        }
    }

    result
}

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn new() -> Self {
        Ingredient {
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        }
    }

    fn from(input: &str) -> Self {
        let input = input.replace(',', "");
        let parts: Vec<&str> = input.split(' ').collect();
        Ingredient {
            capacity: parts[2].parse::<i32>().unwrap(),
            durability: parts[4].parse::<i32>().unwrap(),
            flavor: parts[6].parse::<i32>().unwrap(),
            texture: parts[8].parse::<i32>().unwrap(),
            calories: parts[10].parse::<i32>().unwrap(),
        }
    }

    fn product(&self) -> i32 {
        Self::value(self.capacity)
            * Self::value(self.durability)
            * Self::value(self.flavor)
            * Self::value(self.texture)
    }

    fn value(x: i32) -> i32 {
        if x < 0 {
            return 0;
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve("input_test.txt", false);
        assert_eq!(result, 62842880);
    }

    #[test]
    fn two_works() {
        let result = solve("input_test.txt", true);
        assert_eq!(result, 57600000);
    }
}
