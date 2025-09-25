use crate::error::*;
use crate::types::*;

pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ParseError::Empty);
    }

    let normalized = trimmed
        .replace("+", " + ")
        .replace("-", " - ")
        .replace("*", " * ")
        .replace("x", " x ")
        .replace("X", " X ")
        .replace("/", " / ");
    let tokens = normalized.split_whitespace().collect::<Vec<_>>();

    if tokens.len() != 3 {
        return Err(ParseError::BadFormat);
    }

    let lhs = parse_number(tokens[0])?;
    let op = parse_op(tokens[1])?;
    let rhs = parse_number(tokens[2])?;

    Ok(Expr { lhs, op, rhs })
}

fn parse_number(s: &str) -> Result<f64, ParseError> {
    s.parse().map_err(|_| ParseError::BadFormat)
}

fn parse_op(s: &str) -> Result<Op, ParseError> {
    match s {
        "+" => Ok(Op::Add),
        "-" => Ok(Op::Sub),
        "*" | "x" | "X" => Ok(Op::Mul),
        "/" => Ok(Op::Div),
        _ => Err(ParseError::BadOperator(s.to_string())),
    }
}
