pub fn solve_one(mut p1_pos: usize, mut p2_pos: usize) -> usize {
    let mut p1 = 0usize;
    let mut p2 = 0usize;

    let mut dice = 1usize;

    let mut p1_on_turn = true;

    let mut rolls = 0usize;

    loop {
        let fields = dice * 3 + 3;

        rolls += 3;

        if p1_on_turn {
            p1_pos += fields;
            p1_pos = reduce(p1_pos, 10);
            p1 += p1_pos;

            if p1 >= 1000 {
                break;
            }
        } else {
            p2_pos += fields;
            p2_pos = reduce(p2_pos, 10);
            p2 += p2_pos;

            if p2 >= 1000 {
                break;
            }
        }

        p1_on_turn = !p1_on_turn;

        dice += 3;
        dice = reduce(dice, 100);
    }

    let min = match p1 <= p2 {
        true => p1,
        false => p2,
    };

    min * rolls
}

pub fn solve_two(p1_pos: usize, p2_pos: usize) -> usize {
    let mut p1_won = 0usize;
    let mut p2_won = 0usize;

    let add: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];

    let mut universes: Vec<Standing> = vec![Standing {
        p1: Person {
            pos: p1_pos,
            points: 0,
        },
        p2: Person {
            pos: p2_pos,
            points: 0,
        },
        on_turn: true,
        count: 1,
    }];

    loop {
        let universe: Standing;
        match universes.pop() {
            Some(s) => {
                universe = s;
            }
            None => {
                break;
            }
        }

        if universe.on_turn {
            let p = universe.p1;
            for i in 3..=9 {
                let pos = p.pos + i;
                let count = add[i - 3] * universe.count;
                let pos = reduce(pos, 10);
                let points = p.points + pos;
                if points >= 21 {
                    p1_won += count;
                } else {
                    universes.push(Standing {
                        p1: Person { pos, points },
                        p2: universe.p2.clone(),
                        on_turn: false,
                        count,
                    });
                }
            }
        } else {
            let p = universe.p2;
            for i in 3..=9 {
                let pos = p.pos + i;
                let count = add[i - 3] * universe.count;
                let pos = reduce(pos, 10);
                let points = p.points + pos;
                if points >= 21 {
                    p2_won += count;
                } else {
                    universes.push(Standing {
                        p1: universe.p1.clone(),
                        p2: Person { pos, points },
                        on_turn: true,
                        count,
                    });
                }
            }
        }
    }

    match p1_won >= p2_won {
        true => p1_won,
        false => p2_won,
    }
}

#[derive(Debug)]
struct Standing {
    p1: Person,
    p2: Person,
    on_turn: bool,
    count: usize,
}

#[derive(Debug, Clone)]
struct Person {
    pos: usize,
    points: usize,
}

fn reduce(number: usize, value: usize) -> usize {
    let result = number % value;
    if result == 0 {
        return value;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{solve_one, solve_two};

    #[test]
    fn one_works() {
        assert_eq!(solve_one(4, 8), 739785);
    }

    #[test]
    fn two_works() {
        assert_eq!(solve_two(4, 8), 444356092776315);
    }
}
