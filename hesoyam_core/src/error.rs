error_chain! {
    types {
        Error, ErrorKind_, ResultExt, Result_;
    }

    foreign_links {
        PostgresError(tokio_postgres::error::Error);
    }

    errors {
        ClientNotFound(dialect: String) {
            description("Client not found")
            display("Unable to get client for `{}` dialect", dialect)
        }

        NotConnected {
            description("Please connect first")
            display("Please client.connect first")
        }

        UnknownRow

        ParseError(e: String) {
            description("FromSql ParseError")
            display("Failed to parse from string: {}", e)
        }
    }

    skip_msg_variant
}

#[allow(unused)]
pub type Result<T> = Result_<T>;

pub type ErrorKind = ErrorKind_;
