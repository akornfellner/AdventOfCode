#[derive(Debug)]
pub struct Command {
    pub on: bool,
    pub x: (i32, i32),
    pub y: (i32, i32),
    pub z: (i32, i32),
}

impl Command {
    pub fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.split(' ').collect();

        let on = matches!(parts[0], "on");

        let coord: Vec<&str> = parts[1].split(',').collect();
        let x: Vec<&str> = coord[0].split('=').collect::<Vec<&str>>()[1]
            .split("..")
            .collect();
        let y: Vec<&str> = coord[1].split('=').collect::<Vec<&str>>()[1]
            .split("..")
            .collect();
        let z: Vec<&str> = coord[2].split('=').collect::<Vec<&str>>()[1]
            .split("..")
            .collect();

        let x = (x[0].parse::<i32>().unwrap(), x[1].parse::<i32>().unwrap());
        let y = (y[0].parse::<i32>().unwrap(), y[1].parse::<i32>().unwrap());
        let z = (z[0].parse::<i32>().unwrap(), z[1].parse::<i32>().unwrap());

        Command { on, x, y, z }
    }
}
