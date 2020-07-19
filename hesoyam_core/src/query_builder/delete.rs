use crate::{Condition, ToSql, Result, PostgresDialect, Dialect};

pub struct DeleteQueryBuilder {
    pub dialect: String,
    pub table_name: String,
    pub conditions: Vec<Condition>,
}

impl ToSql for DeleteQueryBuilder {
    fn to_sql(&self) -> Result<String> {
        let dialect = match self.dialect.as_str() {
            "postgres" => PostgresDialect::from_delete_query_builder(&self),
            d => unimplemented!("{} dialect is not implemented yet", d),
        };

        dialect.to_sql()
    }
}
