use std::any::Any;

pub enum Operator {
    Eq,
    NotEq,
    Lt,
    Lte,
    Gt,
    Gte,
    Like,
    Is,
    IsNot,
    In,
}

pub struct Condition {
    pub name: String,
    pub operator: Operator,
    pub value: Box<dyn Any>,
}
