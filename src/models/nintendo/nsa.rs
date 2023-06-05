
pub struct NintendoSwitchAccount {
    pub client_id: Option<String>,
    pub url_scheme: Option<String>,
    pub nso_app_version: String,
    pub nso_api_token_url: Option<String>,
    pub nso_authorize_url: Option<String>,
    pub nso_session_token_url: Option<String>,
    pub nso_user_me_url: Option<String>,
    pub state: Option<String>,
    pub verify: Option<String>,
    pub authCodeChallenge: Option<String>,
}


/*
    "state": self.state,
    "redirect_uri": "npf%s://auth" % self.client_id,
    "client_id": self.client_id,
    "scope": "openid user user.birthday user.mii user.screenName",
    "response_type": "session_token_code",
    "session_token_code_challenge": self.authCodeChallenge.replace(b"=", b""),
    "session_token_code_challenge_method": "S256",
    "theme": "login_form",
*/

pub struct PayloadAuth {
    pub state: String,
    pub redirect_uri: String,
    pub client_id: String,
    pub scope: String,
    pub response_type: String,
    pub session_token_code_challenge: String,
    pub session_token_code_challenge_method: String,
    pub theme: String,
}

/*
    "client_id": self.client_id,
    "session_token_code": session_token_code,
    "session_token_code_verifier": auth_code_verifier.replace(b"=", b""),
*/

pub struct SessionTokenPayload {
    pub client_id: String,
    pub session_token_code: String,
    pub session_token_code_verifier: String,
}

/*
    "client_id": self.client_id,
    "grant_type": "urn:ietf:params:oauth:grant-type:jwt-bearer-session-token",
    "session_token": session_token,
*/

pub struct ServiceTokenPayload {
    pub client_id: String,
    pub grant_type: String,
    pub session_token: String,
}