use rocket::*;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BTCPrivateKeyResponse {
    pub result: Option<String>,
    pub id: serde_json::Value,
    pub error: serde_json::Value,
}
#[derive(Deserialize, Serialize, Debug, Responder)]
#[response(content_type = "json")]
pub struct BTCPrivateKey {
    pub private_key: Option<String>,
}
