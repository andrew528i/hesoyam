use hesoyam::{Model, Field, ToSql}; // TODO: use hesoyam::prelude::*
use model::*;
use std::collections::HashMap;
use std::any::Any;

mod model;

fn main() {
    println!("table_name: {:#?}", User::table_name());

    // insert
    let users = vec![
        User { name: "John".to_owned(), age: 20 },
        User { name: "Tom".to_owned(), age: 30 },
    ];

    let res_1 = User::save("John".to_owned(), 20).to_sql().unwrap();
    let res_2 = users.save().to_sql().unwrap();

    println!("insert one:{}", res_1);
    println!("insert many: {}", res_2);

    // delete
    let res = User::delete(vec![
        User::field_name.eq(&"John".to_owned()),
        User::field_age.gte(&20),
        User::field_age.lte(&30),
    ]).to_sql().unwrap();

    println!("delete: {}", res);

    // update
    let mut update_set: HashMap<Field, Box<dyn Any>> = HashMap::new();

    update_set.insert(User::field_name, Box::new(String::from("Tom")));

    let res = User::update(vec![
        User::field_name.eq(&"John".to_owned()),
        User::field_age.gt(&20),
    ]).set(update_set).to_sql().unwrap();

    println!("update: {}", res);

    // select
    let res = User::select().filter(vec![User::field_age.gte(&20)]).to_sql().unwrap();

    println!("select: {}", res);
}
