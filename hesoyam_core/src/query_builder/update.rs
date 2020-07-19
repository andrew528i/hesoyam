use std::any::Any;

use crate::{Condition, Dialect, Field, PostgresDialect, Result, ToSql};

pub struct UpdateField<'a> {
    pub field: Field,
    pub value: &'a dyn Any,
}

pub struct UpdateQueryBuilder<'a> {
    pub dialect: String,
    pub table_name: String,
    pub filters: Vec<Condition>,
    pub update_fields: Vec<UpdateField<'a>>
}

impl UpdateQueryBuilder {
    pub fn filter(&mut self, conditions: Vec<Condition>) -> &Self {
        self.filters.extend(conditions);

        self
    }
}

impl ToSql for UpdateQueryBuilder {
    fn to_sql(&self) -> Result<String> {
        let dialect = match self.dialect.as_str() {
            "postgres" => PostgresDialect::from_update_query_builder(&self),
            d => unimplemented!("{} dialect is not implemented yet", d),
        };

        dialect.to_sql()
    }
}
