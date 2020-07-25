extern crate reqwest;
extern crate serde_json;

use self::serde_json::Value;
use std::env;
use dotenv::dotenv;

const BASE_URL: &'static str = "https://api.paystack.co/";

// #[derive(Debug)]
// pub struct Paystack {
//     pub secret_key: String
// }


    // pub fn new(secret_key: String) -> Paystack {
    //     return Paystack {
    //         secret_key
    //     };
    // }
    pub async fn initialize(email: String, amount: String) -> Result<serde_json::value::Value, reqwest::Error> {
        dotenv().ok();
        let key = env::var("PAYSTACK_API_KEY");
        let initiate_url = BASE_URL.to_owned() + "transaction/initialize";
        let response: serde_json::Value = reqwest::Client::new().post(&initiate_url).bearer_auth(format!("{:?}", key)).json(&serde_json::json!({
            "email":email,
            "amount":amount,
        })).send().await?.json().await?;
        return Ok(Value::from(response));
    }

    pub async fn verify_payment(reference: String) -> Result<serde_json::value::Value, reqwest::Error> {
        dotenv().ok();
        let key = env::var("PAYSTACK_API_KEY");
        let verify_url = BASE_URL.to_owned() + "transaction/verify/"+ &*reference;
        let response: serde_json::Value = reqwest::Client::new().get(&verify_url).bearer_auth(format!("{:?}", key)).send().await?.json().await?;
        return Ok(Value::from(response));
    }

    pub async fn charge_card(authorization: String, email: String, amount: String) -> Result<serde_json::value::Value, reqwest::Error> {
        dotenv().ok();
        let key = env::var("PAYSTACK_API_KEY");
        let charge_url = BASE_URL.to_owned() + "transaction/charge_authorization";
        let response: serde_json::Value = reqwest::Client::new().post(&charge_url).bearer_auth(format!("{:?}", key)).json(&serde_json::json!({
        "authorization_code":authorization,
        "amount":amount,
        "email":email
        })).send().await?.json().await?;
        if response["status"] == true {
            return Ok(Value::from(response));
        }
        return Ok(Value::from(response));
    }

    pub async fn resolve_bvn() {}

    pub async fn resolve_account_number() {}

    pub async fn bvn_match() {}