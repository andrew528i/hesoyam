use hesoyam::Model;
use model::*;

mod model;

fn main() {
    let users = vec![
        User { name: "John".to_owned(), age: 20 },
        User { name: "Tom".to_owned(), age: 30 },
    ];

    let res_1 = User::save("John".to_owned(), 20).to_sql();
    let res_2 = users.save().to_sql();

    println!(
        "table_name: {}\ncompiled queries:\n{}\n{}",
        User::table_name(),
        res_1,
        res_2);
}
