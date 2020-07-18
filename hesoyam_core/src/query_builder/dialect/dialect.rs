use crate::Result;

pub trait Dialect {
    fn to_sql(&self) -> Result<String>;

    //create_schema(&self) -> Result<String>;
}
