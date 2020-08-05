# 🎯 Hesoyam ORM

[![MIT License](https://img.shields.io/apm/l/atomic-design-ui.svg?)](https://github.com/andrew528i/hesoyam/blob/dev/LICENSE.md)
[![Version](https://img.shields.io/badge/version-0.1.2-blue.svg)](https://github.com/andrew528i/)
[![Version](https://img.shields.io/badge/Rust-1.45.0-orange.svg)](https://github.com/andrew528i/)

Just another one ORM for for Postgres and Clickhouse. If you found it useful please give us a ⭐ :)

# Installation

`Cargo.toml`:
```toml
[dependencies]
hesoyam = "0.1.5"
```

# Examples

Let's custom model at first.
If we won't pass table_name as macro argument default will be generated.
It assumed that tables already created in db.

`model.rs` be like:
```rust
#[model(dialect = "postgres", table_name = "users")]
pub struct User {
    pub name: String,
    pub age: i32,
    pub weight: f32,
}
```

Now we should provide database config to start work:

```rust
let pg_conf = PostgresConfig::new().
    host("loclahost").
    user("postgres").
    password("qwerty").
    port(5432).
    database("postgres");

let ch_conf = ClickhouseConfig::new().
    schema("http").
    hostname("localhost").
    port(8123);

let mut cm = ClientManager::new().
    add_client(&pg_conf)?.
    add_client(&ch_conf)?;
```

### Insert one and multiple entries

```rust
use hesoyam::prelude::*

// insert one entry
let name = String::from("Thomas");
let age = 20;

User::save(name, age).exec(&mut cm)?;

// do the same with multiple users
let users = vec![
    User { name: "John".to_owned(), age: 20 },
    User { name: "Tom".to_owned(), age: 30 },
    User { name: "Guido".to_owned(), age: 99 },
    User { name: "Rob".to_owned(), age: 199 },
    User { name: "Michael".to_owned(), age: 25 },
    User { name: "Jordan".to_owned(), age: 25 },
    User { name: "Raphael".to_owned(), age: 25 },
];

users.save().exec(&mut cm)?;
```

### Select entries
Pay attention that `From` trait is implemented by `#[model]` macro:

```rust
let res = User::select().filter(vec![
    User::field_age.gte(&20),
    User::field_age.lte(&40),
]).exec(&mut cm)?;

for r in res {
    let u: User = r.into();

    println!("{:#?}", u);
}
```

### Raw queries
Also there are `#[query_result]` macro to simplify converting raw query result to struct:

```rust
let raw_query = r#"
    select
        entity_id,
        max(high) as max_high
    from market_quote
    group by entity_id
    order by entity_id;
"#;

#[query_result]
#[derive(Debug)]
struct Result {
    entity_id: i32,
    avg_high: f32,
}

let res = cm.
    get_client("clickhouse")?.
    query(raw_query)?;

for row in res {
    let res: Result = row.into();

    println!("{:#?}", res);
}
```

For more (update/delete) please go to [examples](https://github.com/andrew528i/hesoyam/blob/dev/examples/crud/src/main.rs) 🙂
