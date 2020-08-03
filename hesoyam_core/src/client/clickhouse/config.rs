use crate::client::DBConfig;

#[derive(Default)]
pub struct ClickhouseConfig<'a> {
    pub hostname: Option<&'a str>,
    pub database: Option<&'a str>,
    pub username: Option<&'a str>,
    pub password: Option<&'a str>,
    pub port: Option<u16>,
    pub schema: Option<&'a str>,
}

impl<'a> ClickhouseConfig<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hostname(mut self, hostname: &'a str) -> Self {
        self.hostname = Some(hostname);

        self
    }

    pub fn database(mut self, database: &'a str) -> Self {
        self.database = Some(database);

        self
    }

    pub fn username(mut self, username: &'a str) -> Self {
        self.username = Some(username);

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

    pub fn schema(mut self, schema: &'a str) -> Self {
        self.schema = Some(schema);

        self
    }
}

impl<'a> DBConfig for ClickhouseConfig<'a> {
    fn dialect(&self) -> &'static str {
        "clickhouse"
    }

    fn dsn(&self) -> String {
        if let Some(username) = self.username {
            if let Some(password) = self.password {
                return format!(
                    "{schema}://{username}:{password}@{hostname}:{port}",
                    schema=self.schema.unwrap(),
                    username=username,
                    password=password,
                    hostname=self.hostname.unwrap(),
                    port=self.port.unwrap());
            }
        }

        format!(
            "{schema}://{hostname}:{port}",
            schema=self.schema.unwrap(),
            hostname=self.hostname.unwrap(),
            port=self.port.unwrap())
    }
}
