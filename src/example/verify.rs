#[path = "../transaction.rs"] mod transaction;

use reqwest::Error;
extern crate reqwest;

pub async fn main() -> Result<(), Error> {
    let verify = transaction::verify_payment("uds0jhyj9x".to_string()).await;
    println!("{:#?}", verify);
    Ok(())
}



