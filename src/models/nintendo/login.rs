use serde::Deserialize;
use serde::Serialize;

use crate::models::imink::{Imink};
use crate::models::nintendo::user_info::UserInfo;

/*
        guid: str,
        user_info: UserInfo,
        user_lang: str,
        access_token: str,
        id_token: str,
*/

pub struct NSOLogin {
    pub(crate) guid: String,
    pub(crate) user_info: UserInfo,
    pub(crate) user_lang: String,
    pub(crate) access_token: String,
    pub(crate) id_token: String,
    pub(crate) timestamp: Option<i64>,
    pub(crate) imink_nso: Option<Imink>,
    pub(crate) url: Option<String>,
    pub(crate) body: Option<RequestBody>,
}

/*
        self.body = {
            "parameter": {
                "f": self._imink_nso.f,
                "naIdToken": self.id_token,
                "timestamp": self._imink_nso.timestamp,
                "requestId": self._imink_nso.request_id,
                "naCountry": self.user_info.country,
                "naBirthday": self.user_info.birthday,
                "language": self.user_info.language,
            },
            "requestId": str(uuid.uuid4()),
        }
 */

/*
#[derive!(Deserialize)]
pub struct UserInfo {
    id: String,
    mii: Option<String>,
    #[serde(rename = "candidateMiis")]
    candidate_miis: Vec<String>,
    region: Option<String>,
    gender: String,
    language: String,
    country: String,
    birthday: String,
    #[serde(rename = "isChild")]
    is_child: bool,
    nickname: String,
    #[serde(rename = "screenName")]
    screen_name: String,
    #[serde(rename = "createdAt")]
    created_at: u32,
    #[serde(rename = "updatedAt")]
    updated_at: u32,
    #[serde(rename = "emailOptedIn")]
    email_opted_in: bool,
    #[serde(rename = "emailVerified")]
    email_verified: bool,
    #[serde(rename = "emailOptedInUpdatedAt")]
    email_opted_in_updated_at: u32,
    #[serde(rename = "analyticsOptedIn")]
    analytics_opted_in: bool,
    #[serde(rename = "analyticsOptedInUpdatedAt")]
    analytics_opted_in_updated_at: u32,
    #[serde(rename = "clientFriendsOptedIn")]
    client_friends_opted_in: bool,
    #[serde(rename = "clientFriendsOptedInUpdatedAt")]
    client_friends_opted_in_updated_at: u32,
    timezone: TimeZone,
    #[serde(rename = "AnalyticsPermissions")]
    analytics_permissions: AnalyticsPermissions,
    #[serde(rename = "eachEmailOptedIn")]
    each_email_opted_in: Option<EachEmailOptedIn>,
}

 */


#[derive(Serialize)]
pub struct Parameter {
    pub(crate) f: String,
    pub(crate) na_id_token: String,
    pub(crate) timestamp: u64,
    pub(crate) request_id: String,
    pub(crate) na_country: String,
    pub(crate) na_birthday: String,
    pub(crate) language: String,
}
// pub(crate) f: Box<str>,
// pub(crate) request_id: Box<str>,
// pub(crate) timestamp: u32,

#[derive(Serialize)]
pub struct RequestBody {
    pub(crate) parameter: Parameter,
    pub(crate) request_id: String,
}
