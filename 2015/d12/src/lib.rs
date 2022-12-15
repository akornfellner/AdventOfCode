use serde_json::{Result, Value};
use std::fs;

pub fn solve(input: &str, two: bool) -> Result<i64> {
    let json = fs::read_to_string(input).unwrap();

    let json: Value = serde_json::from_str(&json)?;

    Ok(sum(&json, two))
}

fn sum(value: &Value, red: bool) -> i64 {
    let mut result = 0i64;

    match value {
        Value::Number(x) => {
            result = x.as_i64().unwrap();
        }
        Value::Array(v) => {
            for i in v {
                result += sum(i, red);
            }
        }
        Value::Object(o) => {
            for v in o.values() {
                result += sum(v, red);

                if let Value::String(s) = v {
                    if *s == "red".to_string() && red {
                        return 0;
                    }
                }
            }
        }
        _ => {}
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve("input_test.txt", false).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn two_works() {
        let result = solve("input_test.txt", true).unwrap();
        assert_eq!(result, 4);
    }
}
