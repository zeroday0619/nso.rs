pub(crate) mod models;
pub(crate) mod imink;
pub(crate) mod nintendo;
pub(crate) mod utils;

use imink::mAPI;
use nintendo::metadata::NSOAppVersion;
use crate::imink::IminkInterface;
use crate::nintendo::metadata::get_nso_app_version;

fn main() {
    let mut imink_api = mAPI { token: "".to_string(), step: 1 };
    imink_api.init();
    let res = imink_api.get_imink();
    match res {
        Ok(_imink) => {
            println!("{:?}", _imink.f);
        }
        _ => {
            println!("ERROR");
        }
    }

    let mut nso_app = NSOAppVersion { url: "https://apps.apple.com/us/app/nintendo-switch-online/id1234806557".parse().unwrap() };
    let res = nso_app.get_app_version();
    println!("{:?}", res.as_str());
}