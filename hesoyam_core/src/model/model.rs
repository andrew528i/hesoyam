use crate::model::Field;

pub trait Model {
    fn table_name() -> String;
    fn fields() -> Vec<Field>;
}

pub struct Relation {
    pub source: String,
    pub destination: String,
}
