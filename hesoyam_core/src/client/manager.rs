use std::collections::HashMap;

use crate::client::{ClickhouseClient, Client, DBConfig, PostgresClient};
use crate::error::*;

type ClientMap = HashMap<String, Box<dyn Client>>; // <dialect, impl Client>

pub struct ClientManager {
    clients: ClientMap,
}

impl ClientManager {
    pub fn new() -> Self {
        Self { clients: HashMap::new() }
    }

    pub fn add_client<T: DBConfig>(mut self, config: &T) -> Result<Self> {
        let mut client: Box<dyn Client> = match config.dialect() {
            "postgres" => Box::new(PostgresClient::new(config)),
            "clickhouse" => Box::new(ClickhouseClient::new(config)),
            dialect => panic!("unknown dielact: {}", dialect),
        };

        client.connect()?;

        self.clients.insert(config.dialect().to_owned(), client);

        Ok(self)
    }

    pub fn get_client(&mut self, dialect: &str) -> Result<&mut Box<dyn Client>> {
        match self.clients.get_mut(dialect) {
            Some(client) => Ok(client),
            None => Err(ErrorKind::ClientNotFound(dialect.to_owned()).into()),
        }
    }
}
