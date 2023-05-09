use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use reqwest::header::ACCEPT_ENCODING;
use reqwest::Error;
use crate::models::imink::{Imink, RequestBody};



pub struct mAPI {
    pub(crate) token: String,
    pub(crate) step: u8
}


impl IminkInterface for mAPI {
    fn init(&mut self) {
        // step 1 = NSO, step 2 = APP
        self.token = self.token.to_string();
    }
    fn get_imink(&mut self) -> Result<Imink, Error> {
        let mut url = format!("https://api.imink.app/f").to_string();
        let client = Client::new();
        let res = client.post(&url)
            .header(
                USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.102 Safari/537.36"
            )
            .header(
                ACCEPT_ENCODING,
                "application/json, charset=utf-8"
            )
            .json(&RequestBody {
                token: self.token.clone(),
                hash_method: self.step,
            }).send()?;
        return res.json();
    }
}
pub trait IminkInterface {
    fn init(&mut self);
    fn get_imink(&mut self) -> Result<Imink, Error>;
}