use std::error::Error as StdError;
use std::fmt;
use std::result::Result as StdResult;

use syn::export::Formatter;

type BoxError = Box<dyn StdError + Send + Sync>;
pub type Result<T> = StdResult<T, BoxError>;

// common error

#[derive(Debug)]
pub enum Error {
    QueryBuilder(BoxError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::QueryBuilder(e) => write!(f, "query builder error: {}", e),
        }
    }
}

impl StdError for Error {}

// query builder error

#[derive(Debug)]
struct QueryBuilderError {
    message: String,
}

impl fmt::Display for QueryBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl StdError for QueryBuilderError {}
