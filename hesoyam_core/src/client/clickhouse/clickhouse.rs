use std::collections::HashMap;

use reqwest::blocking::Client as HTTPClient;

use crate::client::{Client, DBConfig, QueryResult, Row};
use crate::client::clickhouse::row::Row as CRow;
use crate::error::*;

pub struct ClickhouseClient {
    dsn: String,
}

impl ClickhouseClient {
    pub fn new<T: DBConfig>(config: &T) -> Self {
        Self { dsn: config.dsn() }
    }
}

impl Client for ClickhouseClient {
    fn connect(&mut self) -> Result<bool> {
        Ok(true)
    }

    fn query(&mut self, query: &str) -> Result<QueryResult> {
        let http_client = HTTPClient::new();
        let resp = http_client.
            post(&self.dsn).
            body(query.to_owned()).
            send()?;

        let text = resp.text()?;
        let mut rdr = csv::ReaderBuilder::new().
            delimiter(b'\t').
            has_headers(true).
            from_reader(text.as_bytes());

        let mut rows: Vec<Row> = Vec::new();
        let columns: Vec<String> = rdr.headers()?.
            iter().
            map(|f| f.to_owned()).
            collect();

        for row in rdr.records() {
            let row = row?;
            let mut body: HashMap<usize, String> = HashMap::new();

            for (i, c) in row.iter().enumerate() {
                body.insert(i, c.to_owned());
            }

            let r = CRow { body, columns: columns.clone() };

            rows.push(Row { pg_row: None, ch_row: Some(r) });
        }

        let mut iter = rows.into_iter();
        let fetch_row = Box::new(move || iter.next());

        Ok(QueryResult::new(fetch_row))
    }
}
