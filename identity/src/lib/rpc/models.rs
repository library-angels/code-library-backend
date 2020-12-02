use serde::{Deserialize, Serialize};

pub type AuthorizationCode = String;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct User {
    pub id: i32,
    pub sub: String,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub active: bool,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub access_manage_books: bool,
    pub access_manage_roles: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
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
    pub sub: u32,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub iat: u64,
    pub exp: u64,
}
