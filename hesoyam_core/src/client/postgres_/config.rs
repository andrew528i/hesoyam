use crate::client::DBConfig;

#[derive(Default)]
pub struct PostgresConfig<'a> {
    pub host: Option<&'a str>,
    pub user: Option<&'a str>,
    pub password: Option<&'a str>,
    pub port: Option<u16>,
    pub database: Option<&'a str>,
}

impl<'a> PostgresConfig<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn host(mut self, host: &'a str) -> Self {
        self.host = Some(host);
        self
    }

    pub fn user(mut self, user: &'a str) -> Self {
        self.user = Some(user);
        self
    }

    pub fn password(mut self, password: &'a str) -> Self {
        self.password = Some(password);
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn database(mut self, database: &'a str) -> Self {
        self.database = Some(database);
        self
    }
}

impl<'a> DBConfig for PostgresConfig<'a> {
    fn dialect(&self) -> &'static str {
        "postgres"
    }

    fn dsn(&self) -> String {
        format!(
            "host={host} user={user} password={password} port={port} dbname={database}",
            host=self.host.unwrap(),
            user=self.user.unwrap(),
            password=self.password.unwrap(),
            port=self.port.unwrap(),
            database=self.database.unwrap())
    }
}
