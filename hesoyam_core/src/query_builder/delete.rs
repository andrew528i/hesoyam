use crate::Condition;

pub struct DeleteQueryBuilder {
    pub table_name: String,
    pub conditions: Vec<Condition>,
}
