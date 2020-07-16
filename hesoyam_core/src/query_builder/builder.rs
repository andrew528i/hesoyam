use crate::Field;
use crate::query_builder::insert::{InsertQueryBuilder, InsertValue};

pub enum QueryBuilderType {
    Insert(InsertQueryBuilder),
    Select,
    Update,
    Delete,
}

pub struct QueryBuilder {
    pub builder_type: QueryBuilderType,
}

impl QueryBuilder {
    pub fn insert(
        table_name: String,
        fields: Vec<Field>,
        values: Vec<InsertValue>,
    ) -> Self {
        Self {
            builder_type: QueryBuilderType::Insert(InsertQueryBuilder { table_name, fields, values })
        }
    }
}
