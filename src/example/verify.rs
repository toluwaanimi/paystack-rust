use paystack::transaction::Paystack;

use reqwest::Error;
extern crate reqwest;

pub async fn main() -> Result<(), Error> {
    let paystack = Paystack::new("sk_test_e1ae9ea530bb5f099db95b6c447ede5e97784d07".to_string());
    let verify = paystack.verify_payment("uds0jhyj9x".to_string()).await;
    println!("{:#?}", verify);
    Ok(())
}



