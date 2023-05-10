use crate::models::nintendo::metadata::NSOAppVersion;
use reqwest::header::USER_AGENT;
use reqwest::header::ACCEPT_ENCODING;
use reqwest::Error;
use soup::prelude::*;


trait GetNsoAppVersion {
    fn init(&mut self);
    fn get_app_version(&self) -> String;
}


impl GetNsoAppVersion for NSOAppVersion {
    fn init(&mut self) {
        self.url = self.url.parse().unwrap();
    }
    fn get_app_version(&self) -> String {
        let resp = reqwest::blocking::get(self.url.as_str()).unwrap();
        let html = resp.text().unwrap();
        let soup = Soup::new(html.as_str());
        let elt = soup.tag("p").class("whats-new__latest__version").find().expect("failed to find <p> tag");
        let ver = elt.text().replace("Version", "").replace(" ", "");
        return ver;
    }
}

pub trait METADATA {
    fn znca_platform() ->  String;
    fn znca_platform_version() -> String;
    fn znca_version() -> String;
    fn znca_user_agent() -> String;
}

impl METADATA for String {
    fn znca_platform() -> String {
        return "IOS".to_string();
    }
    fn znca_platform_version() -> String {
        return "8.0.0".to_string();
    }
    fn znca_version() -> String {
        let mut nsoapp = NSOAppVersion { url: "https://apps.apple.com/us/app/nintendo-switch-online/id1234806557".parse().unwrap() };
        nsoapp.init();
        return nsoapp.get_app_version();
    }
    fn znca_user_agent() -> String {
        let mut user_agent = String::new();
        user_agent.push_str("com.nintendo.znca/");
        user_agent.push_str(&*String::znca_version());
        user_agent.push_str("(");
        user_agent.push_str(&*String::znca_platform());
        user_agent.push_str("/");
        user_agent.push_str(&*String::znca_platform_version());
        user_agent.push_str(")");
        return user_agent;
    }
}