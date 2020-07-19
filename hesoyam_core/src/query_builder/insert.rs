use std::any::Any;
use std::collections::HashMap;

use crate::{Field, ToSql, PostgresDialect, Result, Dialect};

pub type InsertValue = HashMap<Field, Box<dyn Any>>;

pub struct InsertQueryBuilder {
    pub dialect: String,
    pub table_name: String,
    pub fields: Vec<Field>,
    pub values: Vec<InsertValue>,
}

impl ToSql for InsertQueryBuilder {
    fn to_sql(&self) -> Result<String> {
        let dialect = match self.dialect.as_str() {
            "postgres" => PostgresDialect::from_insert_query_builder(&self),
            d => unimplemented!("{} dialect is not implemented yet", d),
        };

        dialect.to_sql()
    }
}
