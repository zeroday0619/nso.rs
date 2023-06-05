pub(crate) mod models;
pub(crate) mod imink;
pub(crate) mod nintendo;
pub(crate) mod utils;
use imink::mAPI;
use nintendo::nsa::NintendoSwitchAccount;
use crate::imink::IminkInterface;
use crate::models::nintendo::nsa::PayloadAuth;
use crate::nintendo::metadata::METADATA;
use crate::nintendo::nsa::NSA;


fn main() {
    let mut imink_api = mAPI { token: "".to_string(), step: 1 };
    imink_api.init();
    let res = imink_api.get_imink();
    match res {
        Ok(_imink) => {
            println!("{:?}", _imink.f);
            println!("{:?}", _imink.timestamp);
            println!("{:?}", _imink.request_id);
        }
        Err(e) => {
            println!("{e:?}");
        }
    }
    let meta = String::znca_user_agent();
    println!("{:?}", meta.to_string());

    let mut nsa = NintendoSwitchAccount {
        client_id: None,
        url_scheme: None,
        nso_app_version: "".to_string(),
        nso_api_token_url: None,
        nso_authorize_url: None,
        nso_session_token_url: None,
        nso_user_me_url: None,
        state: None,
        verify: None,
        authCodeChallenge: None,
    };
    nsa.init();
    let payload = nsa.payload_auth();
    println!("{:?}", payload.client_id);
    println!("{:?}", payload.redirect_uri);
    println!("{:?}", payload.state);
    println!("{:?}", payload.scope);
    println!("{:?}", payload.response_type);
    println!("{:?}", payload.session_token_code_challenge);
    println!("{:?}", payload.session_token_code_challenge_method);
    println!("{:?}", payload.theme);
}