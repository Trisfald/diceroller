use std::str::FromStr;

enum Op {
    Addition,
    Subtraction,
}

#[derive(Copy, Clone)]
pub struct NumberExpression(u8);

impl std::fmt::Display for NumberExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<NumberExpression> for u8 {
    fn from(item: NumberExpression) -> Self {
        item.0
    }
}

impl FromStr for NumberExpression {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split_inclusive(&['+', '-'][..]).collect();
        let mut result = 0;
        let mut op = Op::Addition;
        for x in v {
            let (i, next_op) = if x.trim().ends_with('+') {
                (
                    x[..(x.len() - 1)]
                        .trim()
                        .parse::<u8>()
                        .map_err(|_| format!("bad input: {} [error in: {}]", s, x))?,
                    Op::Addition,
                )
            } else if x.trim().ends_with('-') {
                (
                    x[..(x.len() - 1)]
                        .trim()
                        .parse::<u8>()
                        .map_err(|_| format!("bad input: {} [error in: {}]", s, x))?,
                    Op::Subtraction,
                )
            } else {
                (
                    x.trim()
                        .parse::<u8>()
                        .map_err(|_| format!("bad input: {} [error in: {}]", s, x))?,
                    Op::Addition,
                )
            };
            match op {
                Op::Addition => {
                    result += i;
                }
                Op::Subtraction => {
                    if i > result {
                        return Err(format!("integer overflow in expression: {}", s));
                    }
                    result -= i;
                }
            }
            op = next_op;
        }
        Ok(NumberExpression(result))
    }
}
