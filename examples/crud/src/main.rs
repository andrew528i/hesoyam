use hesoyam::{Dialect, PostgresDialect};
use hesoyam::model;

#[model(dialect = "postgres", table_name = "users")]
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    // println!("table name is: {:}", User::table_name());
    // println!("name field: {:#?}", User::field_name);
    // println!("fields are: {:#?}", User::fields());

    // let qb = User::insert("John".to_owned(), 20);
    let users = vec![
        User { name: "John".to_owned(), age: 20 },
        User { name: "Tom".to_owned(), age: 30 },
    ];

    let qb = users.save();
    let pg = PostgresDialect::new(qb);
    let res = pg.to_sql().unwrap();

    println!("{}", res);
}
