use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::session::jwt::Jwt;

pub type OauthAuthorizationCode = String;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct User {
    pub id: Uuid,
    pub sub: String,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub active: bool,
    pub role_id: Uuid,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct OauthClientIdentifier {
    pub identifier: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SessionToken {
    pub token: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SessionInfo {
    pub sub: Uuid,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub iat: i64,
    pub exp: i64,
}

impl From<Jwt> for SessionInfo {
    fn from(jwt: Jwt) -> Self {
        Self {
            sub: jwt.sub,
            given_name: jwt.given_name,
            family_name: jwt.family_name,
            picture: jwt.picture,
            iat: jwt.iat,
            exp: jwt.exp,
        }
    }
}
