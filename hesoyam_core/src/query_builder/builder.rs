use crate::{Field, InsertQueryBuilder, InsertValue, DeleteQueryBuilder, Condition, Result};

pub enum QueryBuilderType<'a> {
    Insert(&'a InsertQueryBuilder),
    Select,
    Update,
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

    // pub fn to_sql(&self) -> String {
    //     let dialect = match self.dialect.as_str() {
    //         "postgres" => PostgresDialect::new(self),
    //         d => unimplemented!("{} dialect is not implemented yet", d),
    //     };
    //
    //     dialect.to_sql().unwrap()
    // }
}
