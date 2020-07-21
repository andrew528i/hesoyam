use crate::Condition;

pub struct WhereClause {
    pub conditions: Vec<Condition>,
}

impl Default for WhereClause {
    fn default() -> Self {
        Self { conditions: vec![] }
    }
}
