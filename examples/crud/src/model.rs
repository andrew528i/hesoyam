use hesoyam::model;

#[model(dialect = "postgres", table_name = "users")]
#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: u8,
}
