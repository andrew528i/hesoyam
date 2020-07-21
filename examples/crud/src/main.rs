use hesoyam::Model; // TODO: use hesoyam::prelude::*
use model::*;

mod model;

fn main() {
    // let s = "John".to_owned();
    // let mut v: HashMap<Field, &dyn std::any::Any> = HashMap::new();
    // v.insert(User::field_name, &s);
    // v.insert(User::field_age, &(20 as u8));
    //
    // let mut qb = QueryBuilder::insert("postgres".to_owned()).
    //     model(User::table_name(), User::fields()).
    //     values(vec![v]).
    //     to_sql().unwrap();
    //
    // println!("{}", qb);

    println!("table_name: {:#?}", User::table_name());

    // insert
    let users = vec![
        User { name: "John".to_owned(), age: 20 },
        User { name: "Tom".to_owned(), age: 30 },
    ];

    let res_1 = User::save("John".to_owned(), 20).query;
    let res_2 = users.save().query;

    println!("insert one:\t{}", res_1);
    println!("insert many:\t{}", res_2);

    // delete
    let res = User::delete(vec![
        User::field_name.eq(&"John".to_owned()),
        User::field_age.gte(&20),
        User::field_age.lte(&30),
    ]).query;

    println!("delete: {}", res);

    // // update
    // let res = User::update(vec![
    //     User::field_name.eq(&"John".to_owned()),
    //     User::field_age.gt(&20),
    // ]);
    //
    // println!("{}", res);
}
