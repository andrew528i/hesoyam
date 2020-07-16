use std::any::Any;
use std::collections::HashMap;

use crate::Field;

pub type InsertValue = HashMap<Field, Box<dyn Any>>;

pub struct InsertQueryBuilder {
    pub table_name: String,
    pub fields: Vec<Field>,
    pub values: Vec<InsertValue>,
}
