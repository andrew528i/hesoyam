use std::any::Any;
use std::collections::HashMap;

use hesoyam::error::*;
use hesoyam::Field;
use hesoyam::prelude::*;

use crate::model::*;

pub fn execute() -> Result<()> {
    println!("table_name: {:#?}", User::table_name());

    // insert
    let users = vec![
        User::new("John".to_owned(), 20),
        User::new("Tom".to_owned(), 30),
    ];

    let res_1 = User::save("John".to_owned(), 20).to_sql().unwrap();
    let res_2 = users.save().to_sql().unwrap();

    println!("insert one: {}", res_1);
    println!("insert many: {}", res_2);

    // delete
    let res = User::delete(vec![
        User::field_name.eq(&"John".to_owned()),
        User::field_age.gte(&20),
        User::field_age.lte(&30),
    ]).to_sql().unwrap();

    println!("delete query: {}", res);

    // update
    let res = User::update(vec![
        User::field_name.eq(&"John".to_owned()),
        User::field_age.gt(&20),
    ]).set(User::field_name, &"Tom".to_owned()).to_sql().unwrap();

    println!("update query: {}", res);

    // select
    let res = User::select().filter(vec![User::field_age.gte(&20)]).to_sql().unwrap();

    println!("select query: {}", res);

    Ok(())
}