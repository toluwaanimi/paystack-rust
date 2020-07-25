#[path = "../transaction.rs"] mod transaction;
extern crate reqwest;

extern crate serde_json;

use reqwest::Error;

pub async fn main() -> Result<(), Error> {
    let charge_card = transaction::charge_card("AUTH_vmgpwk73jl".to_string(), "a@a.com".to_string(), "5000".to_string()).await;
    println!("{:#?}", charge_card);
    Ok(())
}