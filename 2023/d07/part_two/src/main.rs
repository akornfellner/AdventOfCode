use std::cmp::Ordering;

fn main() {
    let p2 = solve("input.in");
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> i32 {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut hands = input.lines().map(Hand::new).collect::<Vec<Hand>>();
    let mut result = 0;

    hands.sort();
    hands.reverse();

    for (i, hand) in hands.iter().enumerate() {
        result += hand.bid * (i as i32 + 1);
    }

    result
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: i32,
    typ: Type,
}

impl Hand {
    fn new(line: &str) -> Self {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let bid = parts[1].parse::<i32>().unwrap();
        let cards = parts[0].chars().map(Card::from).collect::<Vec<Card>>();
        let typ = Hand::get_type(&cards);

        Hand { cards, bid, typ }
    }

    fn get_type(cards: &[Card]) -> Type {
        let cards = cards.to_vec();
        let mut counts = [0; 13];
        let mut typ = Type::HighCard;
        let mut j = 0;

        for card in cards {
            if card == Card::J {
                j += 1;
            } else {
                counts[card as usize] += 1;
            }
        }

        counts.sort();
        counts.reverse();

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

        for _ in 0..j {
            let typ_new = match typ {
                Type::Five => Type::Five,
                Type::Four => Type::Five,
                Type::FullHouse => Type::Four,
                Type::Three => Type::Four,
                Type::TwoPairs => Type::FullHouse,
                Type::Pair => Type::Three,
                Type::HighCard => Type::Pair,
            };
            typ = typ_new;
        }

        typ
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
    Five,
    Four,
    FullHouse,
    Three,
    TwoPairs,
    Pair,
    HighCard,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Card {
    A,
    K,
    Q,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    J,
}

impl Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Invalid card"),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = *self as i32;
        let b = *other as i32;
        a.cmp(&b)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two() {
        assert_eq!(solve("input_test.in"), 5905);
    }
}
