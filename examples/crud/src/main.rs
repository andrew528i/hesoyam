use hesoyam::{Model, ToSql}; // TODO: use hesoyam::prelude::*
use model::*;

mod model;

fn main() {
    // insert
    let users = vec![
        User { name: "John".to_owned(), age: 20 },
        User { name: "Tom".to_owned(), age: 30 },
    ];

    let res_1 = User::save("John".to_owned(), 20).to_sql().unwrap();
    let res_2 = users.save().to_sql().unwrap();

    println!(
        "table_name: {}\ncompiled queries:\n{}\n{}",
        User::table_name(),
        res_1,
        res_2);

    // delete
    let res = User::delete(vec![
        User::field_name.eq(&"John".to_owned()),
        User::field_age.gte(&20),
        User::field_age.lte(&30),
    ]).to_sql().unwrap();

    println!("{}", res);
}
