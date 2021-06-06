use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::*;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BTCAddressResponse {
    pub result: Option<String>,
    pub id: serde_json::Value,
    pub error: serde_json::Value,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct AddressObject {
    pub address: String,
    pub privatekey: String,
}
impl<'r> Responder<'r> for AddressObject {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!("{}:{}", self.address, self.privatekey)))
            .raw_header("address", self.address)
            .raw_header("privatekey", self.privatekey)
            .ok()
    }
}
