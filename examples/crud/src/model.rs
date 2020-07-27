use hesoyam::model;

#[model(dialect = "postgres", table_name = "users")]
#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: i32,
}

#[model(dialect = "clickhouse", table_name = "market_quote")]
#[derive(Debug)]
pub struct MarketQuote {
    pub entity_id: i32,
    pub low: f32,
    pub high: f32,
    pub open: f32,
    pub close: f32,
    pub created_at: String,
}
