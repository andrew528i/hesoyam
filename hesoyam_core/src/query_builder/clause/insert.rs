use std::any::Any;
use std::collections::HashMap;

use crate::Field;

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
