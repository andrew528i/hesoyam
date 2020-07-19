use crate::{Field, Dialect, PostgresDialect, InsertQueryBuilder, InsertValue, DeleteQueryBuilder, Condition};

pub enum QueryBuilderType {
    Insert(InsertQueryBuilder),
    Select,
    Update,
    Delete(DeleteQueryBuilder),
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

    pub fn delete(
        dialect: String,
        table_name: String,
        conditions: Vec<Condition>,
    ) -> Self {
        Self {
            builder_type: QueryBuilderType::Delete(
                DeleteQueryBuilder { table_name, conditions }),
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
