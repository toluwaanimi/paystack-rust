extern crate reqwest;

use paystack::transaction::Paystack;

extern crate serde_json;

use reqwest::Error;

pub async fn main() -> Result<(), Error> {
    let paystack = Paystack::new("sk_test_xxxxxxxxzzzzz.to_string());
    let transaction = paystack.initialize("a@a.com".to_string(), "5000".to_string()).await;
    println!("{:#?}", transaction);
    Ok(())
}
