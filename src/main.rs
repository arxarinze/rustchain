#![feature(proc_macro_hygiene, decl_macro)]

use base64::encode;
use dotenv;
use rocket::*;
use rocket_contrib::json::Json;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
const BTCNODE: &'static str = "http://3.8.39.172:18332/";
#[get("/")]
fn index() -> &'static str {
    return "The World Is Yours!";
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BTCAddressResponse {
    result: Option<String>,
    id: serde_json::Value,
    error: serde_json::Value,
}
#[derive(Deserialize, Serialize, Debug, Responder)]
#[response(content_type = "json")]
struct BTCAddress {
    address: Option<String>,
}

#[tokio::main]
#[get("/address/<currency>")]
async fn create_address(
    currency: &http::RawStr,
) -> std::result::Result<Json<BTCAddress>, reqwest::Error> {
    let mut btc_result: BTCAddressResponse = BTCAddressResponse {
        result: std::option::Option::Some(String::new()),
        id: json!(""),
        error: json!(""),
    };
    if currency == "BTC" {
        let request_url = format!("{}", BTCNODE);
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
    let response: BTCAddress = BTCAddress {
        address: btc_result.result,
    };
    return Ok(Json(response));
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, create_address])
        .launch();
}
