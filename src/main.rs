#![feature(proc_macro_hygiene, decl_macro)]
use base64::encode;
use dotenv;
use rocket::*;
use rocket_contrib::json::Json;
use serde_json::json;
mod middleware;
mod models;
pub use models::address::BTCAddress;
pub use models::address::BTCAddressResponse;
const BTCNODE: &'static str = "http://3.8.39.172:18332/";
#[get("/")]
fn index() -> &'static str {
    return "The World Is Yours!";
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
    let response: BTCAddress = BTCAddress {
        address: btc_result.result,
    };
    return Ok(Json(response));
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api", routes![create_address])
        .launch();
}
