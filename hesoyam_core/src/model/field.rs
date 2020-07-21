use crate::{FieldType, Condition, Operator};
use std::any::Any;
use syn::export::Debug;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Field {
    // using &str against String because field_%s is const
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

    pub fn not_eq<T: Any + Clone>(&self, value: &T) -> Condition {
        Condition {
            name: self.name.to_owned(),
            operator: Operator::NotEq,
            value: Box::new(value.clone()),
        }
    }

    pub fn lt<T: Any + Clone>(&self, value: &T) -> Condition {
        Condition {
            name: self.name.to_owned(),
            operator: Operator::Lt,
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

    pub fn gt<T: Any + Clone>(&self, value: &T) -> Condition {
        Condition {
            name: self.name.to_owned(),
            operator: Operator::Gt,
            value: Box::new(value.clone()),
        }
    }

    pub fn gte<T: Any + Clone>(&self, value: &T) -> Condition {
        Condition {
            name: self.name.to_owned(),
            operator: Operator::Gte,
            value: Box::new(value.clone()),
        }
    }

    pub fn like<T: Any + Clone>(&self, value: &T) -> Condition {
        Condition {
            name: self.name.to_owned(),
            operator: Operator::Like,
            value: Box::new(value.clone()),
        }
    }

    pub fn is<T: Any + Clone>(&self, value: &T) -> Condition {
        Condition {
            name: self.name.to_owned(),
            operator: Operator::Is,
            value: Box::new(value.clone()),
        }
    }

    pub fn is_not<T: Any + Clone>(&self, value: &T) -> Condition {
        Condition {
            name: self.name.to_owned(),
            operator: Operator::IsNot,
            value: Box::new(value.clone()),
        }
    }
}
