use reqwest::header::USER_AGENT;
use reqwest::header::ACCEPT_ENCODING;
use reqwest::Error;
use soup::prelude::*;

pub trait get_nso_app_version {
    fn init(&mut self);
    fn get_app_version(&self) -> String;
}

pub struct NSOAppVersion {
    pub(crate) url: String,
}

impl get_nso_app_version for NSOAppVersion {
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

trait METADATA {
    fn ZNCA_PLATFORM() -> &'static str;
    fn ZNCA_PLATFORM_VERSION() -> &'static str;
}

impl METADATA for String {
    fn ZNCA_PLATFORM() -> &'static str {
        return "IOS"
    }
    fn ZNCA_PLATFORM_VERSION() -> &'static str {
        return "8.0.0"
    }
}