
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