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
    pub(crate) f: String,
    pub(crate) request_id: String,
    pub(crate) timestamp: u64,
}