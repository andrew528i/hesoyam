use hesoyam::client::ClientManager;
use hesoyam::error::*;

use crate::model::*;

pub fn execute(cm: &mut ClientManager) -> Result<()> {
    // select from postgres
    let res = User::select().filter(vec![
        User::field_age.gte(&20),
        User::field_age.lte(&40),
    ]).exec(cm)?;

    for r in res {
        let u: User = r.into();
        println!("{:#?}", u);
    }

    // select from clickhouse
    let res = MarketQuote::select().filter(vec![
        MarketQuote::field_high.gte(&1500.95),
        MarketQuote::field_high.lte(&1501),
    ]).exec(cm)?;

    for r in res {
        let mq: MarketQuote = r.into();
        println!("{:#?}", mq);
    }

    Ok(())

    // insert multimple to postgres
    // let users = vec![
    //     User { name: "John".to_owned(), age: 20 },
    //     User { name: "Tom".to_owned(), age: 30 },
    //     User { name: "Guido".to_owned(), age: 99 },
    //     User { name: "Rob".to_owned(), age: 199 },
    //     User { name: "Michael".to_owned(), age: 25 },
    //     User { name: "Jordan".to_owned(), age: 25 },
    //     User { name: "Raphael".to_owned(), age: 25 },
    // ];
    //
    // users.save().exec(cm);

    // insert one to postgres
    // User::save("John".to_owned(), 20).exec(cm);
}
