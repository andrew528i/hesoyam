pub trait DBConfig {
    fn dialect(&self) -> &'static str;
    fn dsn(&self) -> String;
}
