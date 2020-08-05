use hesoyam::client::{ClickhouseConfig, ClientManager, PostgresConfig};
use hesoyam::error::*;
use crate::model::Entity;

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
        hostname("").
        username("").
        password("").
        port(8123);

    let mut cm = ClientManager::new().
        add_client(&pg_conf)?.
        add_client(&ch_conf)?;

    // example_raw_query::execute(&mut cm)?;
    // example_model::execute(&mut cm)?;
    // example_model_impl::execute()?;

    let client = cm.get_client("postgres")?;
    let res = client.query("select * from entity")?;

    for r in res {
        let e: Entity = r.into();
        println!("{:#?}", e);
    }

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => println!("It seems all is OK :)"),
        Err(e) => println!("Got an error: {}", e),
    };
}
