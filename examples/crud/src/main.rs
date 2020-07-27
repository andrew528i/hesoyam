use hesoyam::client::{ClickhouseConfig, ClientManager, PostgresConfig};
use hesoyam::error::*;

mod example_model_impl;

mod example_model;
mod example_raw_query;
mod model;

fn run() -> Result<()> {
    let pg_conf = PostgresConfig::new().
        host("").
        user("").
        password("").
        port(5432).
        database("");

    let ch_conf = ClickhouseConfig::new().
        schema("http").
        hostname("localhost").
        port(8123);

    let mut cm = ClientManager::new().
        add_client(&pg_conf)?.
        add_client(&ch_conf)?;

    example_raw_query::execute(&mut cm)?;
    example_model::execute(&mut cm)?;
    example_model_impl::execute()?;

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => println!("It seems all is OK :)"),
        Err(e) => println!("Got an error: {}", e),
    };
}
