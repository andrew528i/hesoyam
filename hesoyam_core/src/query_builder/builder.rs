use crate::{Field, InsertQueryBuilder, InsertValue, DeleteQueryBuilder, Condition, Result, UpdateQueryBuilder, UpdateField};

pub enum QueryBuilderType<'a> {
    Insert(&'a InsertQueryBuilder),
    Select,
    Update(&'a UpdateQueryBuilder),
    Delete(&'a DeleteQueryBuilder),
}

pub struct QueryBuilder {}

pub trait ToSql {
    fn to_sql(&self) -> Result<String>;
}

impl QueryBuilder {
    pub fn insert(
        dialect: String,
        table_name: String,
        fields: Vec<Field>,
        values: Vec<InsertValue>,
    ) -> InsertQueryBuilder {
        InsertQueryBuilder { dialect, table_name, fields, values }
    }

    pub fn delete(
        dialect: String,
        table_name: String,
        conditions: Vec<Condition>,
    ) -> DeleteQueryBuilder {
        DeleteQueryBuilder { dialect, table_name, conditions }
    }

    pub fn update(
        dialect: String,
        table_name: String,
        update_fields: Vec<UpdateField>,
    ) -> UpdateQueryBuilder {
        UpdateQueryBuilder {
            dialect,
            table_name,
            update_fields,
            filters: vec![],
        }
    }
}
