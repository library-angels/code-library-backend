use std::net::SocketAddr;
use tarpc::context;
use super::service::*;

#[derive(Clone)]
pub struct IdentityService(pub SocketAddr);

#[tarpc::server]
impl Identity for IdentityService {
    /// Returns an user
    async fn user(self, _: context::Context, _user_id: u32) -> Result<User, Error> {
        unimplemented!();
    }

    /// Returns a list of users
    async fn users(
        self,
        _: context::Context,
        _offset: u32,
        _limit: u32,
        _active: Option<bool>,
    ) -> Result<Vec<User>, Error> {
        unimplemented!();
    }

    /// Returns a role
    async fn role(self, _: context::Context, _role_id: u32) -> Result<Role, Error> {
        unimplemented!();
    }

    /// Returns a list of roles
    async fn roles(
        self,
        _: context::Context,
        _offset: u32,
        _limit: u32,
    ) -> Result<Vec<Role>, Error> {
        unimplemented!();
    }

    /// Returns an user role
    async fn user_role(self, _: context::Context, _user_role_id: u32) -> Result<UserRole, Error> {
        unimplemented!();
    }

    /// Returns a list of users
    async fn user_roles(
        self,
        _: context::Context,
        _offset: u32,
        _limit: u32,
        _name: Option<u32>
    ) -> Result<Vec<UserRole>, Error> {
        unimplemented!();
    }

    /// Assigns a role to an user account
    async fn user_role_update(
        self,
        _: context::Context,
        _user_id: u32,
        _role_id: u32,
    ) -> Result<(), Error> {
        unimplemented!();
    }

    /// Switches the status of an user account between enabled and disabled
    async fn user_status_update(self, _: context::Context, _user_id: u32, _status: bool) -> Result<(), Error> {
        unimplemented!();
    }

    /// Returns an OAuth 2.0 client identifier.
    async fn oauth_client_identifier(
        self,
        _: context::Context,
    ) -> Result<OauthClientIdentifier, Error> {
        unimplemented!();
    }

    /// Returns a session token and creates a user account
    async fn oauth_authentication(
        self,
        _: context::Context,
        _code: AuthorizationCode,
    ) -> Result<SessionToken, Error> {
        unimplemented!();
    }

    /// Returns the validity and content of a session token
    async fn session_info(
        self,
        _: context::Context,
        _token: String,
    ) -> Result<SessionInfo, Error> {
        unimplemented!();
    }
}
