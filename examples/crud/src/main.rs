use std::any::Any;

use hesoyam::{Dialect, PostgresDialect};
use hesoyam::model;

#[model(dialect = "postgres", table_name = "users")]
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    // let u = User { name: "John".to_owned(), age: 20 };
    // let name = String::from("John");

    // println!("table name is: {:}", User::table_name());
    // println!("name field: {:#?}", User::field_name);
    // println!("fields are: {:#?}", User::fields());

    // let mut value: HashMap<_, Box<dyn Any>> = HashMap::new();
    // value.insert(User::field_name, Box::new("John".to_owned()));
    // value.insert(User::field_age, Box::new(20 as u8));
    //
    // let values = vec![value];
    // let qb = QueryBuilder::insert(User::table_name(), User::fields(), values);
    // let pg = PostgresDialect::new(QueryBuilderType::Insert(qb));
    // let res = pg.to_sql().unwrap();

    // let qb = User::insert("John".to_owned(), 20);
    let users = vec![
        User { name: "John".to_owned(), age: 20 },
        User { name: "Tom".to_owned(), age: 30 },
    ];
    let qb = users.insert_many();
    let pg = PostgresDialect::new(qb);
    let res = pg.to_sql().unwrap();

    println!("{}", res);
}
