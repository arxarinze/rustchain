use rocket::*;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug, Responder)]
#[response(content_type = "json")]
pub struct TxResponse {
    pub txHash: Option<String>,
}

#[derive(Debug, PartialEq, PartialOrd, Deserialize)]
pub struct Transfer {
    pub sender: String,
    pub receiver: String,
    pub amount: f32,
    pub privateKey: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RawTx {
    pub txid: String,
    pub vout: i32,
}
