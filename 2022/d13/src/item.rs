use serde_json::Value;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    List(Vec<Item>),
    Number(i64),
}

impl Item {
    pub fn new(input: &str) -> Self {
        let value: Value = serde_json::from_str(input).unwrap();
        Self::from_value(&value)
    }

    fn from_value(value: &Value) -> Self {
        let mut list: Vec<Item> = vec![];

        match value {
            Value::Number(x) => {
                return Self::Number(x.as_i64().unwrap());
            }
            Value::Array(x) => {
                for y in x {
                    list.push(Self::from_value(y));
                }
            }
            _ => {}
        }

        Self::List(list)
    }

    fn int_to_list(&self) -> Self {
        if let Item::Number(_) = self {
            return Self::List(vec![self.clone()]);
        }

        self.clone()
    }

    pub fn compare(&self, other: &Self) -> Ordering {
        match self {
            Item::Number(x) => match other {
                Item::Number(y) => {
                    return x.cmp(y);
                }
                Item::List(_) => {
                    return self.int_to_list().compare(other);
                }
            },
            Item::List(left) => match other {
                Item::Number(_) => {
                    return self.compare(&other.int_to_list());
                }
                Item::List(right) => {
                    let mut l = left.iter();
                    let mut r = right.iter();

                    loop {
                        let left = l.next();
                        let right = r.next();

                        match left {
                            None => match right {
                                None => break,
                                Some(_) => {
                                    return Ordering::Less;
                                }
                            },
                            Some(x) => match right {
                                None => {
                                    return Ordering::Greater;
                                }
                                Some(y) => {
                                    let result = x.compare(y);
                                    if let Ordering::Equal = result {
                                        continue;
                                    } else {
                                        return result;
                                    }
                                }
                            },
                        }
                    }
                }
            },
        }
        Ordering::Equal
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}
