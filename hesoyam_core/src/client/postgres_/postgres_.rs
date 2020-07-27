use postgres;
use tokio_postgres::row::Row as PRow;

use crate::client::{Client, DBConfig, QueryResult, Row};
use crate::error::*;

pub struct PostgresClient {
    conn: Option<postgres::Client>,
    dsn: String,
}

impl PostgresClient {
    pub fn new<T: DBConfig>(config: &T) -> Self {
        Self { conn: None, dsn: config.dsn() }
    }
}

impl Client for PostgresClient {
    fn connect(&mut self) -> Result<bool> {
        let conn = postgres::Client::connect(
            self.dsn.as_str(),
            postgres::NoTls)?;

        let is_closed = conn.is_closed();
        self.conn = Some(conn);

        Ok(!is_closed)
    }

    fn query(&mut self, query: &str) -> Result<QueryResult> {
        let result = match self.conn.as_mut() {
            Some(client) => client.query(query, &[])?,
            None => return Err(ErrorKind::NotConnected.into())
        };

        let mut iter = result.into_iter();

        let fetch_row = Box::new(move || {
            let next = iter.next();

            match next {
                Some(r) => Some(r.into()),
                None => None,
            }
        });

        Ok(QueryResult::new(fetch_row))
    }
}

impl<'a> From<PRow> for Row {
    fn from(row: PRow) -> Self {
        Row { pg_row: Some(row), ch_row: None }
    }
}
