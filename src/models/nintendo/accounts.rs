use serde::Deserialize;
use serde::Serialize;

/*
from typing import Optional

from pydantic import BaseModel, Field


class _friendCode(BaseModel):
    regenerable: bool
    regenerableAt: int
    _id: str = Field(..., alias="id")


class _membership(BaseModel):
    active: bool


class _nintendoAccount(BaseModel):
    membership: _membership


class _links(BaseModel):
    nintendoAccount: _nintendoAccount
    friendCode: _friendCode


class _permissions(BaseModel):
    presence: str


class _presence(BaseModel):
    state: str
    updatedAt: int
    logoutAt: int
    game: dict


class _user(BaseModel):
    _id: int = Field(..., alias="id")
    nsaId: str
    imageUri: str
    name: str
    supportId: str
    isChildRestricted: bool
    etag: str
    links: _links
    permissions: _permissions
    presence: _presence


class _webApiServerCredential(BaseModel):
    accessToken: str
    expiresIn: int


class _firebaseCredential(BaseModel):
    accessToken: str
    expiresIn: int


class _accounts(BaseModel):
    user: _user
    webApiServerCredential: _webApiServerCredential
    firebaseCredential: _firebaseCredential


class Accounts(BaseModel):
    status: int
    result: Optional[_accounts]
    errorMessage: Optional[str]
    correlationId: str


class Login(BaseModel):
    login: Accounts | None
    time: float


class ServiceToken(BaseModel):
    expires_in: int
    id_token: str
    access_token: str
    scope: list[str]
    token_type: str
*/

#[derive(Deserialize)]
struct FriendCode {
    regenerable: bool,
    #[serde(rename = "regenerableAt")]
    regenerable_at: i64,
    id: String
}

#[derive(Deserialize)]
struct Membership {
    active: bool
}

#[derive(Deserialize)]
struct NintendoAccount {
    membership: Membership
}

#[derive(Deserialize)]
struct Links {
    #[serde(rename = "nintendoAccount")]
    nintendo_account: NintendoAccount,
    #[serde(rename = "friendCode")]
    friend_code: FriendCode
}

#[derive(Deserialize)]
struct Permissions {
    presence: String
}

#[derive(Deserialize)]
struct Presence {
    state: String,
    #[serde(rename = "updatedAt")]
    updated_at: i64,
    #[serde(rename = "logoutAt")]
    logout_at: i64,
    game: Vec<String>
}

#[derive(Deserialize)]
struct User {
    id: u8,
    #[serde(rename = "nsaId")]
    nsa_id: String,
    #[serde(rename = "imageUri")]
    image_uri: String,
    name: String,
    #[serde(rename = "supportId")]
    support_id: String,
    #[serde(rename = "isChildRestricted")]
    is_child_restricted: bool,
    etag: String,
    links: Links,
    permissions: Permissions,
    presence: Presence
}

#[derive(Deserialize)]
struct WebApiServerCredential {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "expiresIn")]
    expires_in: i64
}

#[derive(Deserialize)]
struct FirebaseCredential {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "expiresIn")]
    expires_in: i64
}

#[derive(Deserialize)]
struct Credentials {
    user: User,
    #[serde(rename = "webApiServerCredential")]
    web_api_server_credential: WebApiServerCredential,
    #[serde(rename = "firebaseCredential")]
    firebase_credential: FirebaseCredential
}

#[derive(Deserialize)]
pub struct Accounts {
    status: u8,
    result: Option<Credentials>,
    #[serde(rename = "errorMessage")]
    error_message: Option<String>,
    #[serde(rename = "correlationId")]
    correlation_id: String
}

#[derive(Deserialize)]
pub struct Login {
    login: Option<Accounts>,
    time: f64
}

#[derive(Deserialize)]
pub struct ServiceToken {
    expires_in: i64,
    id_token: String,
    access_token: String,
    scope: Vec<String>,
    token_type: String
}
