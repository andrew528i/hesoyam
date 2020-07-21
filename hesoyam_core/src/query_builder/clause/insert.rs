use crate::Field;
use std::collections::HashMap;
use std::any::Any;

pub type InsertValue = HashMap<Field, Box<dyn Any>>;

pub struct InsertClause {
    pub table_name: String,
    pub fields: Vec<Field>,
    pub values: Vec<InsertValue>
}

impl Default for InsertClause {
    fn default() -> Self {
        Self {
            table_name: "".to_owned(),
            fields: vec![],
            values: vec![],
        }
    }
}
