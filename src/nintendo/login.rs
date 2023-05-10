use crate::mAPI;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::imink::IminkInterface;
use crate::models::nintendo::accounts::Accounts;
use crate::models::nintendo::login::NSOLogin;
use crate::models::nintendo::login::RequestBody;
use crate::models::nintendo::login::Parameter;
use crate::nintendo::metadata::METADATA;

use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use reqwest::header::ACCEPT_ENCODING;

use reqwest::Error;


trait NSOLoginInterface {
    fn init(&mut self);
    fn to_account(&mut self) -> Result<Accounts, Error>;
}

impl NSOLoginInterface for NSOLogin {
    fn init(&mut self) {
        self.url = Option::from(
            "https://api-lp1.znc.srv.nintendo.net/v3/Account/Login".to_string()
        );
        self.timestamp = Option::from(
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
        );
        let mut imink_api = mAPI { token: self.id_token.to_string(), step: 1 };
        imink_api.init();

        let imink_response = imink_api.get_imink();
        match imink_response {
            Ok(_imink) => {
                self.imink_nso = Option::from(_imink);
            }
            Err(e) => {
                panic!("{e:?}");
            }
        };

        self.body = Option::from(
            RequestBody {
                parameter: Parameter {
                    f: self.imink_nso.as_ref().unwrap().f.to_string(),
                    na_id_token: self.id_token.to_string(),
                    timestamp: self.imink_nso.as_ref().unwrap().timestamp,
                    request_id: self.imink_nso.as_ref().unwrap().request_id.to_string(),
                    na_country: self.user_info.country.to_string(),
                    na_birthday:self.user_info.birthday.to_string(),
                    language: self.user_info.language.to_string(),
                },
                request_id: uuid::Uuid::new_v4().to_string(),
            }
        );
    }
    fn to_account(&mut self) -> Result<Accounts, Error> {
        let client = Client::new();
        let response = client.post(self.url.as_ref().unwrap().to_string())
            .header("X-Platform", String::znca_platform())
            .header("X-ProductVersion", String::znca_version())
            .header(USER_AGENT, String::znca_user_agent())
            .json(&self.body).send()?;
        return response.json();
    }
}