use paystack::transaction::Paystack;
use reqwest::Error;
extern crate reqwest;
// const BASE_URL: &'static str = "https://api.paystack.co/";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let paystack = Paystack::new("sk_live_03310dd7b26fd3755d9ec998518f31d7183c91c1".to_string());
    let value = paystack.initialize("a@a.com".to_string(), "5000".to_string()).await;
    println!("{:#?}", value);
    Ok(())
}



