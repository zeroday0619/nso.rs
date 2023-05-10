use serde::Deserialize;
use serde::Serialize;


/*
from pydantic import BaseModel, Field


class _internalAnalysis(BaseModel):
    permitted: bool
    updatedAt: int


class _targetMarketing(BaseModel):
    updatedAt: int
    permitted: bool


class _analyticsPermissions(BaseModel):
    InternalAnalysis: _internalAnalysis
    TargetMarketing: _targetMarketing


class _timezone(BaseModel):
    _id: str = Field(..., alias="id")
    name: str
    utcOffsetSeconds: int
    utcOffset: str


class _deals(BaseModel):
    optedIn: bool
    updatedAt: int


class _survey(BaseModel):
    optedIn: bool
    updatedAt: int


class _eachEmailOptedIn(BaseModel):
    deals: _deals
    survey: _survey


class UserInfo(BaseModel):
    _id: str = Field(..., alias="id")
    mii: None
    candidateMiis: list
    region: None
    gender: str
    language: str
    country: str
    birthday: str
    isChild: bool
    nickname: str
    screenName: str
    createdAt: int
    updatedAt: int
    emailOptedIn: bool
    emailVerified: bool
    emailOptedInUpdatedAt: int
    analyticsOptedIn: bool
    analyticsOptedInUpdatedAt: int
    clientFriendsOptedIn: bool
    clientFriendsOptedInUpdatedAt: int
    timezone: _timezone
    AnalyticsPermissions: _analyticsPermissions
    eachEmailOptedIn: _eachEmailOptedIn | None
*/

#[derive(Deserialize)]
struct InternalAnalysis {
    permitted: bool,
    updated_at: u32,
}

#[derive(Deserialize)]
struct TargetMarketing {
    updated_at: u32,
    permitted: bool,
}

#[derive(Deserialize)]
struct AnalyticsPermissions {
    #[serde(rename = "internalAnalysis")]
    internal_analysis: InternalAnalysis,
    #[serde(rename = "targetMarketing")]
    target_marketing: TargetMarketing,
}

#[derive(Deserialize)]
struct TimeZone {
    id: String,
    name: String,
    utc_offset_seconds: u32,
    utc_offset: String,
}

#[derive(Deserialize)]
struct Deals {
    opted_in: bool,
    updated_at: u32,
}

#[derive(Deserialize)]
struct Survey {
    opted_in: bool,
    updated_at: u32,
}

#[derive(Deserialize)]
struct EachEmailOptedIn {
    deals: Deals,
    survey: Survey,
}

#[derive(Deserialize)]
pub struct UserInfo {
    id: String,
    mii: Option<String>,
    #[serde(rename = "candidateMiis")]
    candidate_miis: Vec<String>,
    region: Option<String>,
    gender: String,
    pub(crate) language: String,
    pub(crate) country: String,
    pub(crate) birthday: String,
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
