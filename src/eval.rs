use crate::error::*;
use crate::types::*;

pub fn evaluate(expr: Expr) -> Result<f64, EvalError> {
    match expr.op {
        Op::Add => Ok(expr.lhs + expr.rhs),
        Op::Sub => Ok(expr.lhs - expr.rhs),
        Op::Mul => Ok(expr.lhs * expr.rhs),
        Op::Div => {
            if expr.rhs == 0.0 {
                Err(EvalError::DivideByZero)
            } else {
                Ok(expr.lhs / expr.rhs)
            }
        }
    }
}
