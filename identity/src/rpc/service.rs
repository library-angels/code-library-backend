#[tarpc::service]
pub trait Identity {
    async fn user(user_id: u32) -> Result<User, Error>;
    async fn users(offset: u32, limit: u32, user_active: Option<bool>) -> Result<Vec<User>, Error>;
    async fn role(role_id: u32) -> Result<Role, Error>;
    async fn roles(offset: u32, limit: u32) -> Result<Vec<Role>, Error>;
    async fn user_role(user_role_id: u32) -> Result<UserRole, Error>;
    async fn user_roles(offset: u32, limit: u32, name: Option<u32>) -> Result<Vec<UserRole>, Error>;
    async fn user_role_update(user_id: u32, role_id: u32) -> Result<(), Error>;
    async fn user_status_update(user_id: u32, status: bool) -> Result<(), Error>;
    async fn oauth_client_identifier() -> Result<OauthClientIdentifier, Error>;
    async fn oauth_authentication(code: AuthorizationCode) -> Result<SessionToken, Error>;
    async fn session_info(token: String) -> Result<SessionInfo, Error>;
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    NotFound,
    AlreadyExists,
    InvalidInput,
    InvalidData,
    InternalError,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub sub: String,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub active: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OauthClientIdentifier {
    pub identifier: String,
}

pub type AuthorizationCode = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionToken {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionInfo {
    pub sub: u32,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub iat: u64,
    pub exp: u64,
    pub role: u32,
}
