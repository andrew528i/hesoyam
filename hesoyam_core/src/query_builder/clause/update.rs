use crate::Field;
use std::any::Any;
use std::collections::HashMap;

pub type SetValue = HashMap<Field, Box<dyn Any>>;

pub struct UpdateClause {
    pub table_name: String,
    pub values: SetValue,
}

impl Default for UpdateClause {
    fn default() -> Self {
        Self {
            table_name: "".to_owned(),
            values: HashMap::new(),
        }
    }
}
