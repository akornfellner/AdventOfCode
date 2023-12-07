use std::cmp::Ordering;

fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (i32, i32) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut hands1 = input
        .lines()
        .map(|h| Hand::new(h, false))
        .collect::<Vec<Hand>>();
    let mut hands2 = input
        .lines()
        .map(|h| Hand::new(h, true))
        .collect::<Vec<Hand>>();
    let mut result = (0, 0);

    hands1.sort();
    hands2.sort();

    for (i, hand) in hands1.iter().enumerate() {
        result.0 += hand.bid * (i as i32 + 1);
        result.1 += hands2[i].bid * (i as i32 + 1);
    }

    result
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<i32>,
    bid: i32,
    typ: Type,
}

impl Hand {
    fn new(line: &str, two: bool) -> Self {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let bid = parts[1].parse::<i32>().unwrap();
        let cards = parts[0]
            .chars()
            .map(|c| Hand::get_card(c, two))
            .collect::<Vec<i32>>();
        let typ = Hand::get_type(&cards);

        Hand { cards, bid, typ }
    }

    fn get_type(cards: &[i32]) -> Type {
        let cards = cards.to_vec();
        let mut counts = [0; 13];
        let mut typ = Type::HighCard;
        let mut j = 0;

        for card in cards {
            if card < 0 {
                j += 1;
            } else {
                counts[card as usize] += 1;
            }
        }

        counts.sort();
        counts.reverse();

        counts[0] += j;

        if counts[0] == 5 {
            typ = Type::Five;
        } else if counts[0] == 4 {
            typ = Type::Four;
        } else if counts[0] == 3 && counts[1] == 2 {
            typ = Type::FullHouse;
        } else if counts[0] == 3 {
            typ = Type::Three;
        } else if counts[0] == 2 && counts[1] == 2 {
            typ = Type::TwoPairs;
        } else if counts[0] == 2 {
            typ = Type::Pair;
        }

        typ
    }

    fn get_card(c: char, two: bool) -> i32 {
        match c {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'J' => {
                if two {
                    -1
                } else {
                    9
                }
            }
            'T' => 8,
            _ => c.to_digit(10).unwrap() as i32 - 2,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.typ != other.typ {
            return self.typ.cmp(&other.typ);
        }
        for (i, card) in self.cards.iter().enumerate() {
            if *card != other.cards[i] {
                return card.cmp(&other.cards[i]);
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Type {
    HighCard,
    Pair,
    TwoPairs,
    Three,
    FullHouse,
    Four,
    Five,
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = *self as i32;
        let b = *other as i32;
        a.cmp(&b)
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve("input_test.in").0, 6440);
    }

    #[test]
    fn test_two() {
        assert_eq!(solve("input_test.in").1, 5905);
    }
}
