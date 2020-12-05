use std::net::SocketAddr;
use std::sync::Arc;

use chrono::Utc;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use tarpc::context;

use helpers::rpc::{Error, RpcResult};

use super::{models::*, service::IdentityService};
use crate::authentication::{create_user_from_oauth_authentication, oauth};
use crate::config::Configuration;
use crate::db::{queries, DbPool};
use crate::session::jwt::Jwt;

#[derive(Clone)]
pub struct IdentityServer {
    addr: SocketAddr,
    conf: Arc<Configuration>,
    db_pool: Arc<DbPool>,
}

impl IdentityServer {
    pub fn new(addr: SocketAddr, conf: Arc<Configuration>, db_pool: Arc<DbPool>) -> Self {
        Self {
            addr,
            conf,
            db_pool,
        }
    }

    fn get_db(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
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
                access_manage_books: val.access_manage_books,
                access_manage_roles: val.access_manage_roles,
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
                    access_manage_books: x.access_manage_books,
                    access_manage_roles: x.access_manage_roles,
                })
                .collect::<Vec<Role>>()),
            Err(diesel::result::Error::NotFound) => Err(Error::NotFound),
            Err(_) => Err(Error::InternalError),
        }
    }

    /// Returns an user role
    async fn get_user_role(self, _: context::Context, user_role_id: u32) -> RpcResult<UserRole> {
        let result = queries::get_user_role(user_role_id as i32, &self.get_db());

        match result {
            Ok(val) => Ok(UserRole {
                id: val.id,
                user_id: val.user_id,
                role_id: val.role_id,
            }),
            Err(diesel::result::Error::NotFound) => Err(Error::NotFound),
            Err(_) => Err(Error::InternalError),
        }
    }

    /// Returns a list of users
    async fn list_user_roles(
        self,
        _: context::Context,
        offset: u32,
        limit: u32,
        role_id: Option<u32>,
    ) -> RpcResult<Vec<UserRole>> {
        let results = match role_id {
            Some(val) => queries::list_user_roles(
                offset.into(),
                limit.into(),
                Some(val as i32),
                &self.get_db(),
            ),
            None => queries::list_user_roles(offset.into(), limit.into(), None, &self.get_db()),
        };

        match results {
            Ok(val) => Ok(val
                .iter()
                .map(|x| UserRole {
                    id: x.id,
                    user_id: x.user_id,
                    role_id: x.role_id,
                })
                .collect::<Vec<UserRole>>()),
            Err(diesel::result::Error::NotFound) => Err(Error::NotFound),
            Err(_) => Err(Error::InternalError),
        }
    }

    /// Assigns a role to an user account
    async fn update_user_role(
        self,
        _: context::Context,
        user_role_update: UserRole,
    ) -> RpcResult<UserRole> {
        let result = queries::update_user_role(user_role_update.into(), &self.get_db());

        match result {
            Ok(val) => Ok(UserRole {
                id: val.id,
                user_id: val.user_id,
                role_id: val.role_id,
            }),
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
        code: AuthorizationCode,
    ) -> RpcResult<SessionToken> {
        let authorization_code = oauth::AuthorizationCode::new(code)?;

        let request = oauth::TokenRequest::new(
            authorization_code,
            self.conf.oauth_client_identifier.clone(),
            self.conf.oauth_client_secret.clone(),
            oauth::RedirectUri::PostMessage,
            oauth::GrantType::AuthorizationCode,
        );

        let tokenset = request
            .exchange_code(oauth::DiscoveryDocument::get_token_endpoint())
            .await?;

        let id_token = oauth::IdToken::new(&tokenset.id_token)?;

        use crate::authentication::{check_account_status, AccountStatus};
        let account_status =
            check_account_status(queries::get_user_by_sub(&id_token.sub, &self.get_db()))?;

        if account_status == AccountStatus::Inactive {
            log::info!("Rejected inactive account \"{}\"", &id_token.email);
            return Err(Error::InvalidInput);
        } else if account_status == AccountStatus::New && tokenset.refresh_token.is_none() {
            log::info!("Rejected new account \"{}\"", &id_token.email);
            log::info!("Missing refresh token for account \"{}\"", &id_token.email);
            return Err(Error::InvalidInput);
        }

        let user =
            create_user_from_oauth_authentication(&id_token, &tokenset, Utc::now().naive_utc());

        let user = match account_status {
            AccountStatus::Active => queries::update_user_by_sub(user, &self.get_db())?,
            AccountStatus::New => queries::create_user(user, &self.get_db())?,
            _ => return Err(Error::InternalError),
        };
        log::info!(
            "Successfully created/updated account \"{}\"",
            &id_token.email
        );

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
