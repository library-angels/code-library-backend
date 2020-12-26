use std::sync::Arc;

use chrono::{Duration, Utc};
use tarpc::context;

use helpers::rpc::{Error, RpcResult};

use super::{models::*, service::IdentityService};
use crate::authentication::oauth::{
    AuthorizationCode, DiscoveryDocument, GrantType, IdToken, RedirectUri, TokenRequest,
};
use crate::authentication::{
    check_account_status, create_user_from_oauth_authentication, AccountStatus,
};
use crate::config::Configuration;
use crate::db::{queries, DbConn, DbPool};
use crate::session::jwt::Jwt;

#[derive(Clone)]
pub struct IdentityServer {
    conf: Arc<Configuration>,
    db_pool: Arc<DbPool>,
}

impl IdentityServer {
    pub fn new(conf: Arc<Configuration>, db_pool: Arc<DbPool>) -> Self {
        Self { conf, db_pool }
    }

    fn get_db(&self) -> DbConn {
        self.db_pool
            .get()
            .expect("Can't retrieve connection from pool")
    }
}

#[tarpc::server]
impl IdentityService for IdentityServer {
    /// Returns an user
    async fn get_user(self, _: context::Context, user_id: u32) -> RpcResult<User> {
        let result = queries::get_user(user_id as i32, &self.get_db());

        match result {
            Ok(val) => Ok(User {
                id: val.id,
                sub: val.sub,
                email: val.email,
                given_name: val.given_name,
                family_name: val.family_name,
                picture: val.picture,
                active: val.active,
                role_id: val.role_id,
            }),
            Err(diesel::result::Error::NotFound) => Err(Error::NotFound),
            Err(_) => Err(Error::InternalError),
        }
    }

    /// Returns a list of users
    async fn list_users(
        self,
        _: context::Context,
        offset: u32,
        limit: u32,
        user_active: Option<bool>,
    ) -> RpcResult<Vec<User>> {
        let results = queries::list_users(limit.into(), offset.into(), user_active, &self.get_db());

        match results {
            Ok(val) => Ok(val
                .iter()
                .map(|x| User {
                    id: x.id,
                    sub: x.sub.clone(),
                    email: x.email.clone(),
                    given_name: x.given_name.clone(),
                    family_name: x.family_name.clone(),
                    picture: x.picture.clone(),
                    active: x.active,
                    role_id: x.role_id,
                })
                .collect::<Vec<User>>()),
            Err(diesel::result::Error::NotFound) => Err(Error::NotFound),
            Err(_) => Err(Error::InternalError),
        }
    }

    /// Returns a role
    async fn get_role(self, _: context::Context, role_id: u32) -> RpcResult<Role> {
        let result = queries::get_role(role_id as i32, &self.get_db());

        match result {
            Ok(val) => Ok(Role {
                id: val.id,
                name: val.name,
            }),
            Err(diesel::result::Error::NotFound) => Err(Error::NotFound),
            Err(_) => Err(Error::InternalError),
        }
    }

    /// Switches the status of an user account between enabled and disabled
    async fn update_user(self, _: context::Context, user_update: User) -> RpcResult<User> {
        let result = queries::update_user(user_update.into(), &self.get_db());

        match result {
            Ok(val) => Ok(User {
                id: val.id,
                sub: val.sub.clone(),
                email: val.email.clone(),
                given_name: val.given_name.clone(),
                family_name: val.family_name.clone(),
                picture: val.picture.clone(),
                active: val.active,
                role_id: val.role_id,
            }),
            Err(diesel::result::Error::NotFound) => Err(Error::NotFound),
            Err(_) => Err(Error::InternalError),
        }
    }

    /// Returns a list of roles
    async fn list_roles(
        self,
        _: context::Context,
        offset: u32,
        limit: u32,
    ) -> RpcResult<Vec<Role>> {
        let results = queries::list_roles(offset.into(), limit.into(), &self.get_db());

        match results {
            Ok(val) => Ok(val
                .iter()
                .map(|x| Role {
                    id: x.id,
                    name: x.name.clone(),
                })
                .collect::<Vec<Role>>()),
            Err(diesel::result::Error::NotFound) => Err(Error::NotFound),
            Err(_) => Err(Error::InternalError),
        }
    }

    /// Returns an OAuth 2.0 client identifier.
    async fn oauth_client_identifier(
        self,
        _: context::Context,
    ) -> RpcResult<OauthClientIdentifier> {
        Ok(OauthClientIdentifier {
            identifier: self.conf.oauth_client_identifier.clone(),
        })
    }

    /// Returns a session token and creates or updates an user account
    async fn oauth_authentication(
        self,
        _: context::Context,
        code: OauthAuthorizationCode,
    ) -> RpcResult<SessionToken> {
        // Checks if the authorization code has a valid form
        let authorization_code = AuthorizationCode::new(code)?;

        // Creates a TokenRequest instance
        let request = TokenRequest::new(
            authorization_code,
            self.conf.oauth_client_identifier.clone(),
            self.conf.oauth_client_secret.clone(),
            RedirectUri::PostMessage,
            GrantType::AuthorizationCode,
        );

        // Exchanges the TokenRequest for a Tokenset
        let tokenset = request
            .exchange_code(DiscoveryDocument::get_token_endpoint())
            .await?;

        // Creates an IdToken from the TokenSet
        let id_token = IdToken::new(&tokenset.id_token)?;

        // Checks the status of the user about to authenticate
        let account_status =
            check_account_status(queries::get_user_by_sub(&id_token.sub, &self.get_db()))?;

        // Checks if the user account is inactive or authentication has missing refresh token for new account
        if account_status == AccountStatus::Inactive {
            log::info!("Rejected inactive account '{}'", &id_token.email);
            return Err(Error::InvalidInput);
        } else if account_status == AccountStatus::New && tokenset.refresh_token.is_none() {
            log::info!("Rejected new account '{}'", &id_token.email);
            log::info!("Missing refresh token for account '{}'", &id_token.email);
            return Err(Error::InvalidInput);
        }

        // Creates an user model instance from IdToken and TokenSet
        let user =
            create_user_from_oauth_authentication(&id_token, &tokenset, Utc::now().naive_utc());

        // Create or update user record in database
        let user = match account_status {
            AccountStatus::Active => queries::update_user_by_sub(user, &self.get_db())?,
            AccountStatus::New => queries::create_user(user, &self.get_db())?,
            _ => return Err(Error::InternalError),
        };
        log::info!("Successfully created/updated account '{}'", &id_token.email);

        // Create and return SessionToken
        Ok(SessionToken {
            token: Jwt::new(
                user.id as u32,
                user.given_name,
                user.family_name,
                user.picture,
                Utc::now(),
                Duration::seconds(3600),
            )
            .encode(&self.conf.jwt_secret()),
        })
    }

    /// Returns the validity and content of a session token
    async fn session_info(self, _: context::Context, token: String) -> RpcResult<SessionInfo> {
        match Jwt::decode(&self.conf.jwt_secret(), &token) {
            Ok(val) => Ok(val.into()),
            Err(_) => Err(Error::InvalidData),
        }
    }
}
