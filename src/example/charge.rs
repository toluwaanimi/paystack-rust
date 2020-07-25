extern crate reqwest;

use paystack::transaction::Paystack;

extern crate serde_json;

use reqwest::Error;

pub async fn main() -> Result<(), Error> {
    let paystack = Paystack::new("sk_test_e1ae9ea530bb5f099db95b6c447ede5e97784d07".to_string());
    let charge_card = paystack.charge_card("AUTH_vmgpwk73jl".to_string(), "a@a.com".to_string(), "5000".to_string()).await;
    println!("{:#?}", charge_card);
    Ok(())
}