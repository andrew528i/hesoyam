use crate::FieldType;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Field {
    pub name: &'static str,
    pub field_type: FieldType,
}
