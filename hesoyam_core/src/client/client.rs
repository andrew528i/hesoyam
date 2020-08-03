use std::fmt;

use postgres;
use postgres::row::RowIndex;
use postgres::types::FromSql;

use crate::client::clickhouse::row::{FromSql as CFromSql, Row as CRow, RowIndex as CRowIndex};
use crate::error::*;

pub trait Client: Send {
    fn connect(&mut self) -> Result<bool>;
    fn query(&mut self, query: &str) -> Result<QueryResult>;
}

pub struct QueryResult {
    fetch_row_: Box<dyn FnMut() -> Option<Row>>,
}

impl QueryResult {
    pub fn new(fetch_row: Box<dyn FnMut() -> Option<Row>>) -> Self {
        Self { fetch_row_: fetch_row }
    }

    fn fetch_row(&mut self) -> Option<Row> {
        let fetch_row = &mut (self.fetch_row_);

        fetch_row()
    }

    pub fn first(&mut self) -> Option<Row> {
        self.fetch_row()
    }

    pub fn one(&mut self) -> Row {
        self.fetch_row().unwrap()
    }
}

impl Iterator for QueryResult {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        self.fetch_row()
    }
}

pub struct Row {
    pub pg_row: Option<tokio_postgres::row::Row>,
    pub ch_row: Option<CRow>
}

impl Row {
    pub fn get<'b, T: FromSql<'b> + CFromSql, U: RowIndex + fmt::Display + CRowIndex>(&'b self, key: U) -> Result<T> {
        if let Some(row) = self.pg_row.as_ref() {
            return Ok(row.get::<U, T>(key))
        }

        if let Some(row) = self.ch_row.as_ref() {
            return row.get::<T, U>(key)
        }

        Err(ErrorKind::UnknownRow.into())
    }
}
