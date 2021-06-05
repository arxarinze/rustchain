#![feature(proc_macro_hygiene, decl_macro)]
use amiquip::{
    AmqpValue, Channel, Connection, ConsumerMessage, ConsumerOptions, Exchange, FieldTable,
    Publish, QueueDeclareOptions, Result,
};
use base64::encode;
use dotenv;
use rocket::*;
use rocket_contrib::json::Json;
use serde_json::json;
mod middleware;
mod models;
pub use models::address::BTCAddress;
pub use models::address::BTCAddressResponse;
pub use models::privatekey::BTCPrivateKey;
pub use models::privatekey::BTCPrivateKeyResponse;
use std::thread;

#[get("/")]
fn index() -> &'static str {
    return "The World Is Yours!";
}

async fn getBitcoinAddressPrivateKey(
    address: Option<String>,
) -> std::result::Result<Option<String>, reqwest::Error> {
    let BTCNODE = dotenv::var("BTCNODE").unwrap();
    let mut btc_result: BTCPrivateKeyResponse = BTCPrivateKeyResponse {
        result: std::option::Option::Some(String::new()),
        id: json!(""),
        error: json!(""),
    };
    let request_url = format!("{}wallet/", BTCNODE);
    let USER = dotenv::var("USER").unwrap();
    let PASSWORD = dotenv::var("PASSWORD").unwrap();
    let body = json!({
        "jsonrpc": "1.0",
        "id": "curltest",
        "method": "dumpprivkey",
        "params": [
            address
        ]
    });
    let auth = format!("Basic {}", encode(USER + ":" + &PASSWORD));
    let client = reqwest::Client::new();
    let res = client
        .post(request_url)
        .header("Authorization", auth)
        .json(&body)
        .send()
        .await?;
    let address: BTCPrivateKeyResponse = res.json().await?;
    btc_result = address.clone();
    println!("{:?}", address);

    let response: BTCPrivateKey = BTCPrivateKey {
        private_key: btc_result.result,
    };
    return Ok(response.private_key);
}

#[tokio::main]
#[get("/address/<currency>")]
async fn create_address(
    currency: &http::RawStr,
) -> std::result::Result<Json<BTCAddress>, reqwest::Error> {
    let BTCNODE = dotenv::var("BTCNODE").unwrap();
    let mut btc_result: BTCAddressResponse = BTCAddressResponse {
        result: std::option::Option::Some(String::new()),
        id: json!(""),
        error: json!(""),
    };
    if currency == "BTC" {
        let request_url = format!("{}wallet/", BTCNODE);
        let USER = dotenv::var("USER").unwrap();
        let PASSWORD = dotenv::var("PASSWORD").unwrap();
        let body = json!({
            "jsonrpc": "1.0",
            "id": "curltest",
            "method": "getnewaddress",
            "params": []
        });
        let auth = format!("Basic {}", encode(USER + ":" + &PASSWORD));
        let client = reqwest::Client::new();
        let res = client
            .post(request_url)
            .header("Authorization", auth)
            .json(&body)
            .send()
            .await?;
        let address: BTCAddressResponse = res.json().await?;
        btc_result = address.clone();
        println!("{:?}", address);
    }
    let pk: Option<String> = getBitcoinAddressPrivateKey(btc_result.result.clone()).await?;
    println!("{:?}", pk);
    let re = json!({
        "address": btc_result.result,
        "private_key": pk
    });
    let response: BTCAddress = BTCAddress {
        address: btc_result.result.unwrap(),
        privatekey: pk.unwrap(),
    };
    return Ok(Json(response));
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api", routes![create_address])
        .launch();
}
