use hesoyam::client::ClientManager;
use hesoyam::error::*;
use hesoyam::query_result;

pub fn execute(cm: &mut ClientManager) -> Result<()> {
    // Clickhouse
    let raw_query = r#"
        select
            entity_id,
            avg(high) as avg_high
        from market_quote
        group by entity_id
        order by entity_id;
    "#;

    let res = cm.
        get_client("clickhouse")?.
        query(raw_query)?;

    #[query_result]
    #[derive(Debug)]
    struct ChResult {
        entity_id: i32,
        avg_high: f32,
    }

    for row in res {
        let res: ChResult = row.into();

        println!("{:#?}", res);
    }

    // Postgres
    let raw_query = r#"
        select
            customer_id,
            count(*) as order_count
        from order_order
        group by customer_id
        having count(*) > 0;
    "#;

    let res = cm.
        get_client("postgres")?.
        query(raw_query)?;

    #[query_result]
    #[derive(Debug)]
    struct PgResult {
        customer_id: i32,
        order_count: i64,
    }

    for row in res {
        let res: PgResult = row.into();

        println!("{:#?}", res);
    }

    Ok(())
}
