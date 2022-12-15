use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn solve(input: &str) -> (usize, usize) {
    let (distances, mut places) = get_input(input);

    let mut ways = vec![];
    let size = places.len();
    permut(&mut places, size, &mut ways);

    let mut min = usize::MAX;
    let mut max: usize = usize::MIN;

    for way in ways {
        check_way(&way, &distances, &mut min, &mut max);
    }

    (min, max)
}

fn permut(places: &mut Vec<String>, size: usize, results: &mut Vec<Vec<String>>) {
    if size == 1 {
        results.push(places.clone());
        return;
    }

    for i in 0..size {
        permut(places, size - 1, results);

        if size % 2 == 1 {
            (places[0], places[size - 1]) = (places[size - 1].clone(), places[0].clone());
        } else {
            (places[i], places[size - 1]) = (places[size - 1].clone(), places[i].clone());
        }
    }
}

fn check_way(
    way: &Vec<String>,
    distances: &HashMap<(String, String), usize>,
    min: &mut usize,
    max: &mut usize,
) {
    let mut sum = 0usize;

    for i in 0..way.len() - 1 {
        sum += distances[&sort_tuple(&way[i], &way[i + 1])];
    }

    if sum < *min {
        *min = sum;
    }

    if sum > *max {
        *max = sum;
    }
}

fn get_input(input: &str) -> (HashMap<(String, String), usize>, Vec<String>) {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut distances: HashMap<(String, String), usize> = HashMap::new();
    let mut places: HashSet<String> = HashSet::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" = ").collect();
        let dist = parts[1].parse::<usize>().unwrap();
        let parts: Vec<&str> = parts[0].split(" to ").collect();

        distances.insert(sort_tuple(parts[0], parts[1]), dist);
        places.insert(parts[0].to_string());
        places.insert(parts[1].to_string());
    }

    let mut result: Vec<String> = vec![];

    for place in &places {
        result.push(place.to_string());
    }

    (distances, result)
}

fn sort_tuple(a: &str, b: &str) -> (String, String) {
    if a > b {
        return (b.to_string(), a.to_string());
    }
    (a.to_string(), b.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve("input_test.txt");
        assert_eq!(result, (605, 982));
    }
}
