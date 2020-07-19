use std::any::Any;

#[derive(Debug)]
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
}

#[derive(Debug)]
pub struct Condition {
    pub name: String,
    pub operator: Operator,
    pub value: Box<dyn Any>,
}
