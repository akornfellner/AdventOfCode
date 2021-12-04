use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

pub fn d17(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();

    buf_reader
        .read_to_string(&mut input)
        .expect("Reading Error");

    let mut qubes = cubes_from_input(&input);

    for _round in 0..6 {
        for key in qubes.clone().keys() {
            let neighbors = get_neighbors(key);
            for neighbor in neighbors {
                qubes.entry(neighbor).or_insert(false);
            }
        }

        let state = qubes.clone();

        for qube in qubes.clone() {
            let neighbors = get_neighbors(&qube.0);
            let mut count = 0;
            for neighbor in neighbors {
                if !state.contains_key(&neighbor) || state[&neighbor] == false {
                    ();
                } else {
                    count += 1;
                }
            }

            if qube.1 {
                if count < 2 || count > 3 {
                    qubes.insert(qube.0, false);
                }
            } else {
                if count == 3 {
                    qubes.insert(qube.0, true);
                }
            }
        }
    }

    let mut count = 0;

    for qube in qubes {
        if qube.1 {
            count += 1;
        }
    }

    count
}

fn cubes_from_input(input: &str) -> HashMap<(i32, i32, i32, i32), bool> {
    let rows: Vec<&str> = input.split('\n').collect();
    let mut result: HashMap<(i32, i32, i32, i32), bool> = HashMap::new();
    for i in 0..rows.len() {
        let chars: Vec<char> = rows[i].chars().collect();
        for j in 0..chars.len() {
            if chars[j] == '#' {
                result.insert((i as i32, j as i32, 0, 0), true);
            }
        }
    }

    result
}

fn get_neighbors(cube: &(i32, i32, i32, i32)) -> Vec<(i32, i32, i32, i32)> {
    let mut result: Vec<(i32, i32, i32, i32)> = vec![];

    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                for w in -1..2 {
                    if !(x == 0 && y == 0 && z == 0 && w == 0) {
                        result.push((cube.0 + x, cube.1 + y, cube.2 + z, cube.3 + w));
                    }
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::d17;

    #[test]
    fn it_works() {
        assert_eq!(d17("input_test.txt"), 848);
    }
}
