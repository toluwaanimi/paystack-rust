#[path = "example/initialize.rs"]
mod initialize;
#[path = "example/verify.rs"]
mod verify;
#[path = "example/add_key.rs"]
mod add_keys;
#[path = "example/charge.rs"]
mod charge;

use reqwest::Error;

extern crate reqwest;
#[warn(unused_must_use)]
#[tokio::main]
async fn main() -> Result<(), Error> {
    // initialize::main().await;
    // verify::main().await;
    // charge::main().await;
    Ok(())
}



