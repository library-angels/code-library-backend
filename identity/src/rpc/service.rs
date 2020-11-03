use serde::{Deserialize, Serialize};

pub use helpers::rpc::Error;

#[tarpc::service]
pub trait IdentityService {
    async fn get_user(user_id: u32) -> Result<User, Error>;
    async fn list_users(
        offset: u32,
        limit: u32,
        user_active: Option<bool>,
    ) -> Result<Vec<User>, Error>;
    async fn update_user(user_update: User) -> Result<User, Error>;
    async fn get_role(role_id: u32) -> Result<Role, Error>;
    async fn list_roles(offset: u32, limit: u32) -> Result<Vec<Role>, Error>;
    async fn get_user_role(user_role_id: u32) -> Result<UserRole, Error>;
    async fn list_user_roles(
        offset: u32,
        limit: u32,
        role_id: Option<u32>,
    ) -> Result<Vec<UserRole>, Error>;
    async fn update_user_role(user_role_update: UserRole) -> Result<UserRole, Error>;
    async fn oauth_client_identifier() -> Result<OauthClientIdentifier, Error>;
    async fn oauth_authentication(code: AuthorizationCode) -> Result<SessionToken, Error>;
    async fn session_info(token: String) -> Result<SessionInfo, Error>;
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
    pub access_manage_books: bool,
    pub access_manage_roles: bool,
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
}
