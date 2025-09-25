pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
pub struct Expr {
    pub lhs: f64,
    pub rhs: f64,
    pub op: Op,
}
