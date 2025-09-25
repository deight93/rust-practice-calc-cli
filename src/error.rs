use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("빈 입력입니다.")]
    Empty,
    #[error("형식 오류 입니다. 예: 3 + 5")]
    BadFormat,
    #[error("지원하지 않는 연산자 {0}")]
    BadOperator(String),
}

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("0으로 나눌 수 없습니다.")]
    DivideByZero,
}

#[derive(Error, Debug)]
pub enum CalcError {
    #[error(transparent)]
    ParseError(#[from] ParseError),
    #[error(transparent)]
    EvalError(#[from] EvalError),
}
