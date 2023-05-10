use getrandom::getrandom;
use base64::{Engine as _, engine::general_purpose};
use crate::models::nintendo::nsa::NintendoSwitchAccount;
use sha256::digest;

impl NSA for NintendoSwitchAccount {
    fn init(&mut self) {
        let mut buffer_36 = [0u8; 36];
        let mut buffer_32 = [0u8; 32];

        getrandom(&mut buffer_36).unwrap();
        getrandom(&mut buffer_32).unwrap();
        self.client_id = Option::from("71b963c1b7b6d119".to_string());
        self.url_scheme = Option::from("npf71b963c1b7b6d119".to_string());
        self.nso_api_token_url = Option::from(
            "https://accounts.nintendo.com/connect/1.0.0/api/token".to_string()
        );
        self.nso_authorize_url = Option::from(
            "https://accounts.nintendo.com/connect/1.0.0/authorize".to_string()
        );
        self.nso_session_token_url = Option::from(
            "https://accounts.nintendo.com/connect/1.0.0/api/session_token".to_string()
        );
        self.nso_user_me_url = Option::from(
            "https://api.accounts.nintendo.com/2.0.0/users/me".to_string()
        );
        self.state = Option::from(
            general_purpose::URL_SAFE_NO_PAD.encode(buffer_36)
        );
        self.verify = Option::from(
            general_purpose::URL_SAFE_NO_PAD.encode(buffer_32)
        );
        let mut verify = self.verify.as_ref().unwrap();

        self.authCodeChallenge = Option::from(
            general_purpose::URL_SAFE_NO_PAD.encode(
                digest(verify.replace("=", ""))
            )
        )
    }
}

trait NSA {
    fn init(&mut self);
}