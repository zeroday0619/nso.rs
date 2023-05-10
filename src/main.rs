pub(crate) mod models;
pub(crate) mod imink;
pub(crate) mod nintendo;
pub(crate) mod utils;

use imink::mAPI;
use crate::imink::IminkInterface;
use crate::nintendo::metadata::METADATA;

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
    let meta = String::znca_user_agent();
    println!("{:?}", meta.to_string());
}