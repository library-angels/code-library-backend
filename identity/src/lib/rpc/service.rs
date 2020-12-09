pub use helpers::rpc::{Error, RpcResult};

use super::models::*;

#[tarpc::service]
pub trait IdentityService {
    async fn get_user(user_id: u32) -> RpcResult<User>;
    async fn list_users(offset: u32, limit: u32, user_active: Option<bool>)
        -> RpcResult<Vec<User>>;
    async fn update_user(user_update: User) -> RpcResult<User>;
    async fn get_role(role_id: u32) -> RpcResult<Role>;
    async fn list_roles(offset: u32, limit: u32) -> RpcResult<Vec<Role>>;
    async fn oauth_client_identifier() -> RpcResult<OauthClientIdentifier>;
    async fn oauth_authentication(code: OauthAuthorizationCode) -> RpcResult<SessionToken>;
    async fn session_info(token: String) -> RpcResult<SessionInfo>;
}
