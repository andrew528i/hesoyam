use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

use crate::error::*;

pub struct Row {
    pub(crate) body: HashMap<usize, String>,
    pub(crate) columns: Vec<String>,
}

impl Row {
    pub fn get<T: FromSql, U: RowIndex>(&self, key: U) -> Result<T> {
        let column = key.get_index(&self.columns);
        let v = self.body.get(&column).unwrap().as_str();
        
        FromSql::from_sql(v)
    }
}

// RowIndex
pub trait RowIndex: Hash + Eq {
    fn get_index(&self, columns: &Vec<String>) -> usize;
}

impl RowIndex for usize {
    #[allow(unused_variables)]
    fn get_index(&self, columns: &Vec<String>) -> usize {
        self.clone()
    }
}

impl RowIndex for &str {
    fn get_index(&self, columns: &Vec<String>) -> usize {
        for (i, v) in columns.iter().enumerate() {
            if v == self {
                return i
            }
        }

        return 0
    }
}

// FromSql
pub trait FromSql: Sized + std::str::FromStr {
    fn from_sql(value: &str) -> Result<Self> {
        match value.parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(ErrorKind::ParseError(value.to_owned()).into())
        }
    }
}

impl FromSql for i8 {}

impl FromSql for i16 {}

impl FromSql for i32 {}

impl FromSql for i64 {}

impl FromSql for u8 {}

impl FromSql for u16 {}

impl FromSql for u32 {}

impl FromSql for u64 {}

impl FromSql for f32 {}

impl FromSql for f64 {}

impl FromSql for String {
    fn from_sql(value: &str) -> Result<Self> {
        Ok(value.to_owned())
    }
}

impl FromSql for bool {
    fn from_sql(value: &str) -> Result<Self> {
        match value {
            "0" => Ok(false),
            _ => Ok(true),
        }
    }
}
