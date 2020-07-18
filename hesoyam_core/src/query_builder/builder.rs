use crate::{Field, Dialect, PostgresDialect};
use crate::query_builder::insert::{InsertQueryBuilder, InsertValue};

pub enum QueryBuilderType {
    Insert(InsertQueryBuilder),
    Select,
    Update,
    Delete,
}

pub struct QueryBuilder {
    pub builder_type: QueryBuilderType,
    pub dialect: String,
}

impl QueryBuilder {
    pub fn insert(
        dialect: String,
        table_name: String,
        fields: Vec<Field>,
        values: Vec<InsertValue>,
    ) -> Self {
        Self {
            builder_type: QueryBuilderType::Insert(
                InsertQueryBuilder { table_name, fields, values }),
            dialect,
        }
    }

    pub fn to_sql(&self) -> String {
        let dialect = match self.dialect.as_str() {
            "postgres" => PostgresDialect::new(self),
            d => unimplemented!("{} dialect is not implemented yet", d),
        };

        dialect.to_sql().unwrap()
    }
}
