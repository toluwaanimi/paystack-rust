extern crate reqwest;
extern crate serde_json;

use self::serde_json::Value;
use std::convert::TryFrom;

const BASE_URL: &'static str = "https://api.paystack.co/";

#[derive(Debug)]
pub struct Paystack {
    pub secret_key: String
}

impl Paystack {
    pub fn new(secret_key: String) -> Paystack {
        return Paystack {
            secret_key
        };
    }
    pub async fn initialize(self, email: String, amount: String) -> Result<serde_json::value::Value, reqwest::Error> {
        let key = self.secret_key;
        let initiate_url = BASE_URL.to_owned() + "transaction/initialize";
        let response: serde_json::Value = reqwest::Client::new().post(&initiate_url).bearer_auth(key).json(&serde_json::json!({
            "email":email,
            "amount":amount,
        })).send().await?.json().await?;
        let body = response;
        if body["status"] == true {
            Ok(Value::from(body))
            // println!("{:#?}", body);
        } else {}
    }

    pub async fn verify_payment(self, reference: String) {
        let key = self.secret_key;
        let verify_url = BASE_URL.to_owned() + "transaction/verify/{}" + &*reference;
        let response: serde_json::Value = reqwest::Client::new().get(&verify_url).bearer_auth(key).send().await?.json().await?;
        if response["status"] == true {
            Ok(Value::from(response))
        }
    }

    pub async fn charge_card(self, authorization: String, email: String, amount: String) -> Result<serde_json::value::Value, reqwest::Error> {
        let key = self.secret_key;
        let charge_url = BASE_URL.to_owned() + "transaction/charge_authorization";
        let response: serde_json::Value = reqwest::Client::new().post(&charge_url).bearer_auth(key).json(&serde_json::json!({
        "authorization_code":authorization,
        "amount":amount,
        "email":email
        })).send().await?.json().await?;
    }
}