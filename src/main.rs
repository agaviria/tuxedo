mod conf;
mod cli;

#[macro_use]
extern crate clap;
extern crate futures;
extern crate tokio_core as tokio;
extern crate rustc_serialize;

#[macro_use]
extern crate mysql_async as my;

use futures::Future;
use my::prelude::*;
use tokio::reactor::Core;
use conf::Config;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

fn main() {
    let options = cli::gen_cli().get_matches();

    let mut lp = Core::new().unwrap();

    let payments = vec![
        Payment { customer_id: 1, amount: 2, account_name: None },
        Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
        Payment { customer_id: 5, amount: 6, account_name: None },
        Payment { customer_id: 7, amount: 8, account_name: None },
        Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
    ];

    let cfg = conf::load();
    let pool = my::Pool::new(&*cfg.mysql_uri(), &lp.handle());
    let future = pool.get_conn()
        .and_then(|conn| {
            // Create temporary table
            conn.query(r"CREATE TEMPORARY TABLE tmp.payment (
                customer_id int not null,
                amount int not null,
                account_name text
            )")
                .and_then(|result| result.drop_result())
        })
        .and_then(|conn| {
            // Save payments
            let params = payments.iter()
                .map(|payment| {
                    params! {
                "customer_id" => payment.customer_id,
                "amount" => payment.amount,
                "account_name" => payment.account_name.clone(),
            }
                })
                .collect();

            conn.batch_exec(r"INSERT INTO tmp.payment (customer_id, amount, account_name)
                        VALUES (:customer_id, :amount, :account_name)",
                            params)
        })
        .and_then(|conn| {
            // Load payments from database.
            conn.prep_exec("SELECT customer_id, amount, account_name FROM tmp.payment",
                           ())
        })
        .and_then(|result| {
            // Collect payments
            result.map(|row| {
                let (customer_id, amount, account_name) = my::from_row(row);
                Payment {
                    customer_id: customer_id,
                    amount: amount,
                    account_name: account_name,
                }
            })
        })
        .map(|(payments, _ /* conn */)| {
            // Drop connection
            payments
        });

    let loaded_payments = lp.run(future).unwrap();

    assert_eq!(loaded_payments, payments);
    println!("fin.");
}
