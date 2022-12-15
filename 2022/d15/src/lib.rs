use std::{collections::HashSet, fs};

pub fn solve(input: &str, y: i32, square: i32, two: bool) -> usize {
    let input = fs::read_to_string(input).unwrap();

    let mut beacons: HashSet<(i32, i32)> = HashSet::new();

    let sensors: Vec<Sensor> = input
        .lines()
        .map(|s| Sensor::new(s, &mut beacons))
        .collect();

    if !two {
        let mut minx = i32::MAX;
        let mut maxx = i32::MIN;

        for sensor in &sensors {
            if sensor.sensor.0 - sensor.dist < minx {
                minx = sensor.sensor.0 - sensor.dist;
            }
            if sensor.sensor.0 + sensor.dist > maxx {
                maxx = sensor.sensor.0 + sensor.dist;
            }
        }

        let mut count = 0usize;

        for x in minx..=maxx {
            if beacons.contains(&(x, y)) {
                continue;
            }

            if !check((x, y), &sensors) {
                count += 1;
            }
        }

        return count;
    }

    let result = search_beacon(&sensors, square).unwrap();

    result.0 as usize * 4000000 + result.1 as usize
}

fn check(field: (i32, i32), sensors: &[Sensor]) -> bool {
    for sensor in sensors {
        if sensor.can_see(field) {
            return false;
        }
    }

    true
}

// the beacon we are looking for has to be at distance d+1 to some scanner because otherwise there would be more than one possible solution
fn search_beacon(sensors: &[Sensor], square: i32) -> Option<(i32, i32)> {
    for sensor in sensors {
        let d = sensor.dist + 1;

        for dx in 0..=d {
            let dy = d - dx;

            let fields = [
                (sensor.sensor.0 - dx, sensor.sensor.1 - dy),
                (sensor.sensor.0 - dx, sensor.sensor.1 + dy),
                (sensor.sensor.0 + dx, sensor.sensor.1 - dy),
                (sensor.sensor.0 + dx, sensor.sensor.1 + dy),
            ];

            for field in fields {
                if !(field.0 >= 0 && field.0 <= square && field.1 >= 0 && field.1 <= square) {
                    continue;
                }
                if check(field, sensors) {
                    return Some(field);
                }
            }
        }
    }

    None
}

#[derive(Debug)]
struct Sensor {
    sensor: (i32, i32),
    dist: i32,
}

impl Sensor {
    fn new(input: &str, beacons: &mut HashSet<(i32, i32)>) -> Self {
        let input = input
            .replace(':', "")
            .replace("x=", "")
            .replace("y=", "")
            .replace(',', "");

        let arr: Vec<&str> = input.split(' ').collect();

        let sensor = (
            arr[2].parse::<i32>().unwrap(),
            arr[3].parse::<i32>().unwrap(),
        );

        let beacon = (
            arr[8].parse::<i32>().unwrap(),
            arr[9].parse::<i32>().unwrap(),
        );

        beacons.insert(beacon);

        let dist = distance(sensor, beacon);

        Self { sensor, dist }
    }

    fn can_see(&self, field: (i32, i32)) -> bool {
        distance(self.sensor, field) <= self.dist
    }
}

fn distance(first: (i32, i32), second: (i32, i32)) -> i32 {
    (first.0 - second.0).abs() + (first.1 - second.1).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve("input_test.txt", 10, 20, false);
        assert_eq!(result, 26);
    }

    #[test]
    fn two_works() {
        let result = solve("input_test.txt", 10, 20, true);
        assert_eq!(result, 56000011);
    }
}
