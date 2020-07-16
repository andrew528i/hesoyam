use crate::{QueryBuilder, Result};

pub trait Dialect {
    fn new(query_builder: QueryBuilder) -> Self;
    fn to_sql(&self) -> Result<String>;

    //create_schema(&self) -> Result<String>;
}
