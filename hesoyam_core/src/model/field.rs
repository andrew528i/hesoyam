use crate::{FieldType, Condition, Operator};
use std::any::Any;
use syn::export::Debug;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Field {
    pub name: &'static str,
    pub field_type: FieldType,
}

impl Field {
    // TODO: generate this stuff in macro
    pub fn eq<T: Any + Clone>(&self, value: &T) -> Condition {
        Condition {
            name: self.name.to_owned(),
            operator: Operator::Eq,
            value: Box::new(value.clone()),
        }
    }

    pub fn lte<T: Any + Clone>(&self, value: &T) -> Condition {
        Condition {
            name: self.name.to_owned(),
            operator: Operator::Lte,
            value: Box::new(value.clone()),
        }
    }
}
