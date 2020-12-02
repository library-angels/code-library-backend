use serde::{Deserialize, Serialize};

pub use helpers::rpc::{Error, RpcResult};

#[tarpc::service]
pub trait IdentityService {
    async fn get_user(user_id: u32) -> RpcResult<User>;
    async fn list_users(offset: u32, limit: u32, user_active: Option<bool>)
        -> RpcResult<Vec<User>>;
    async fn update_user(user_update: User) -> RpcResult<User>;
    async fn get_role(role_id: u32) -> RpcResult<Role>;
    async fn list_roles(offset: u32, limit: u32) -> RpcResult<Vec<Role>>;
    async fn get_user_role(user_role_id: u32) -> RpcResult<UserRole>;
    async fn list_user_roles(
        offset: u32,
        limit: u32,
        role_id: Option<u32>,
    ) -> RpcResult<Vec<UserRole>>;
    async fn update_user_role(user_role_update: UserRole) -> RpcResult<UserRole>;
    async fn oauth_client_identifier() -> RpcResult<OauthClientIdentifier>;
    async fn oauth_authentication(code: AuthorizationCode) -> RpcResult<SessionToken>;
    async fn session_info(token: String) -> RpcResult<SessionInfo>;
}

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

pub type AuthorizationCode = String;

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
