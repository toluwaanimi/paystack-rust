#[path = "../transaction.rs"] mod transaction;

extern crate reqwest;

extern crate serde_json;

use reqwest::Error;

pub async fn main() -> Result<(), Error> {
    let transaction = transaction::initialize("a@a.com".to_string(), "5000".to_string()).await;
    println!("{:#?}", transaction);
    Ok(())
}
