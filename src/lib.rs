pub mod error;
pub mod eval;
pub mod parser;
pub mod types;

use error::CalcError;

pub fn compute(expr: &str) -> Result<f64, CalcError> {
    let parsed = parser::parse(expr)?;
    let value = eval::evaluate(parsed)?;
    Ok(value)
}
