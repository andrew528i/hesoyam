use crate::Field;

pub struct SelectClause {
    pub table_name: String,
    pub values: Vec<Selectable>,
}

pub enum Selectable {
    Field(Field),
    All,
    Func,
}

impl From<Field> for Selectable {
    fn from(f: Field) -> Self {
        Self::Field(f)
    }
}

impl SelectClause {
    pub fn from_values(values: Vec<Selectable>) -> Self {
        Self {
            table_name: "".to_owned(),
            values,
        }
    }
}

impl Default for SelectClause {
    fn default() -> Self {
        Self {
            table_name: "".to_owned(),
            values: vec![],
        }
    }
}
