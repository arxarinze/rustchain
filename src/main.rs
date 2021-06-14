#![feature(proc_macro_hygiene, decl_macro)]
// use amiquip::{
//     AmqpValue, Channel, Connection, ConsumerMessage, ConsumerOptions, Exchange, FieldTable,
//     Publish, QueueDeclareOptions, Result,
// };
use base64::encode;
use dotenv;
use rocket::*;
use rocket_contrib::json::Json;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
mod middleware;
mod models;
use chrono::{DateTime, Datelike, Timelike, Utc};
use ethkey::prelude::*;
use ethsign::*;
use hex_slice::AsHex;
pub use models::address::AddressObject;
pub use models::address::BTCAddressResponse;
pub use models::privatekey::BTCPrivateKey;
pub use models::privatekey::BTCPrivateKeyResponse;
pub use models::transaction::RawTx;
pub use models::transaction::Transfer;
pub use models::transaction::TxResponse;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use std::fs;
use std::thread;
pub struct CORS;

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(Debug, Deserialize)]
struct CryptoKey {
    cipher: ethsign::keyfile::Cipher,
    cipherparams: ethsign::keyfile::Aes128Ctr,
    ciphertext: ethsign::keyfile::Bytes,
    kdf: ethsign::keyfile::Kdf,
    mac: ethsign::keyfile::Bytes,
}

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
    let request_url = format!("{}wallet/ipay", BTCNODE);
    let USER = dotenv::var("BTCUSER").unwrap();
    let PASSWORD = dotenv::var("BTCPASSWORD").unwrap();
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
    let response: BTCPrivateKey = BTCPrivateKey {
        private_key: btc_result.result,
    };
    return Ok(response.private_key);
}

#[tokio::main]
#[post(
    "/transfer/<currency>",
    format = "application/json",
    data = "<transfer>"
)]
async fn create_transfer(
    currency: &http::RawStr,
    transfer: Json<Transfer>,
) -> std::result::Result<Json<TxResponse>, reqwest::Error> {
    let BTCNODE = dotenv::var("BTCNODE").unwrap();
    let ETHNODE = dotenv::var("ETHNODE").unwrap();
    let t_obj: Json<Transfer> = transfer;
    println!("{:?}", t_obj);
    let mut response: TxResponse = TxResponse {
        txHash: std::option::Option::Some(String::new()),
    };
    if (currency == "BTC") {
        let request_url = format!("{}wallet/ipay", BTCNODE);
        let USER = dotenv::var("BTCUSER").unwrap();
        let PASSWORD = dotenv::var("BTCPASSWORD").unwrap();
        let auth = format!("Basic {}", encode(USER + ":" + &PASSWORD));
        let body = json!({
            "jsonrpc": "1.0",
            "id": "curltest",
            "method": "listunspent",
            "params": [
                0,
                9999999, [t_obj.sender]
            ]
        });
        println!("{:?}", body);
        let client = reqwest::Client::new();
        let res = client
            .post(request_url)
            .header("Authorization", auth)
            .json(&body)
            .send()
            .await?;

        let test: serde_json::Value = res.json().await?;
        let working: HashMap<String, serde_json::Value> =
            serde_json::from_str(&format!("{}", test)).unwrap();
        let result = working["result"].as_array().unwrap();
        let length = result.len();
        for obj in result.iter() {
            println!("{}", obj)
        }
        let mut rawTx: Vec<RawTx> = Vec::<RawTx>::new();
        println!("{:?}", length);
    }
    return Ok(Json(response));
}

#[tokio::main]
#[get("/address/<currency>")]
async fn create_address(
    currency: &http::RawStr,
) -> std::result::Result<Json<AddressObject>, reqwest::Error> {
    let BTCNODE = dotenv::var("BTCNODE").unwrap();
    let mut btc_result: BTCAddressResponse = BTCAddressResponse {
        result: std::option::Option::Some(String::new()),
        id: json!(""),
        error: json!(""),
    };
    let mut response: AddressObject = AddressObject {
        address: String::new(),
        privatekey: String::new(),
    };
    if currency == "BTC" {
        let request_url = format!("{}wallet/ipay", BTCNODE);
        let USER = dotenv::var("BTCUSER").unwrap();
        let PASSWORD = dotenv::var("BTCPASSWORD").unwrap();
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
        let pk: Option<String> = getBitcoinAddressPrivateKey(btc_result.result.clone()).await?;
        let re = json!({
            "address": btc_result.result,
            "private_key": pk
        });
        response = AddressObject {
            address: btc_result.result.unwrap(),
            privatekey: pk.unwrap(),
        };
    } else if currency == "ETH" {
        let now: DateTime<Utc> = Utc::now();
        let passphrase = format!("ipayBTCWallet{}", now).replace(" ", "");
        let fname = format!("ipayBTCWallet{}", now.second());
        let filename = format!("./keystore/{}.json", fname);

        let key_store = EthAccount::load_or_generate(filename.clone(), passphrase.clone())
            .expect("should load or generate new eth key");
        let address = key_store.address();
        let file = std::fs::File::open(filename.clone()).unwrap();
        let key: KeyFile = serde_json::from_reader(file).unwrap();
        let password: Protected = passphrase.clone().into();
        let crypto: ethsign::keyfile::Crypto = key.crypto;
        let keyByte: Vec<u8> = crypto.decrypt(&password).unwrap();
        // let str = std::string::String::from_utf8();
        let pk = format!("0x{:02x}", keyByte.as_hex());
        let pk_r = format!("{}", pk.replace("[", "").replace("]", "").replace(" ", ""));
        response = AddressObject {
            address: format!("{}", address),
            privatekey: pk_r,
        };
        delete_keystore(filename);
    }

    return Ok(Json(response));
}
fn delete_keystore(filename: String) -> std::io::Result<()> {
    fs::remove_file(&filename)?;
    Ok(())
}

fn main() {
    rocket::ignite()
        .attach(CORS)
        .mount("/", routes![index])
        .mount("/api", routes![create_address, create_transfer])
        .launch();
}
