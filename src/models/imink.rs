use serde::Deserialize;
use serde::Serialize;


#[derive(Serialize)]
pub struct RequestBody {
    pub token: String,
    #[serde(rename = "hashMethod")]
    pub hash_method: u8,
}


#[derive(Deserialize)]
pub struct Imink {
    pub(crate) f: Box<str>,
    pub(crate) request_id: Box<str>,
    pub(crate) timestamp: u32,
}