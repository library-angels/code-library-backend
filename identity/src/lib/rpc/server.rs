use std::net::SocketAddr;
use std::sync::Arc;

use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use tarpc::context;

use helpers::rpc::{Error, RpcResult};

use super::{models::*, service::IdentityService};
use crate::authentication::oauth;
use crate::config::Configuration;
use crate::db::{models, DbPool};
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
        use crate::db::schema::users::dsl;

        let result = dsl::users
            .find(user_id as i32)
            .first::<models::User>(&self.get_db());

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
        use crate::db::schema::users::dsl;

        let results = match user_active {
            Some(val) => dsl::users
                .filter(dsl::active.eq(val))
                .offset(offset as i64)
                .limit(limit as i64)
                .load::<models::User>(&self.get_db()),
            None => dsl::users
                .offset(offset as i64)
                .limit(limit as i64)
                .load::<models::User>(&self.get_db()),
        };

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
        use crate::db::schema::roles::dsl;

        let result = dsl::roles
            .find(role_id as i32)
            .first::<models::Role>(&self.get_db());

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
        use crate::db::schema::users::dsl;

        let result = diesel::update(dsl::users.find(user_update.id as i32))
            .set(dsl::active.eq(user_update.active))
            .get_result::<models::User>(&self.get_db());

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
        use crate::db::schema::roles::dsl;

        let results = dsl::roles
            .offset(offset as i64)
            .limit(limit as i64)
            .load::<models::Role>(&self.get_db());

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
        use crate::db::schema::users_roles::dsl;

        let result = dsl::users_roles
            .find(user_role_id as i32)
            .first::<models::UserRole>(&self.get_db());

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
        use crate::db::schema::users_roles::dsl;

        let results = match role_id {
            Some(val) => dsl::users_roles
                .filter(dsl::role_id.eq(val as i32))
                .offset(offset as i64)
                .limit(limit as i64)
                .load::<models::UserRole>(&self.get_db()),
            None => dsl::users_roles
                .offset(offset as i64)
                .limit(limit as i64)
                .load::<models::UserRole>(&self.get_db()),
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
        use crate::db::schema::users_roles::dsl;

        let result = diesel::update(dsl::users_roles.find(user_role_update.id as i32))
            .set(dsl::role_id.eq(user_role_update.role_id as i32))
            .get_result::<models::UserRole>(&self.get_db());

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
        use crate::db::schema::users::dsl::*;

        let authorization_code = match oauth::AuthorizationCode::new(code) {
            Ok(val) => val,
            Err(_) => return Err(Error::InvalidData),
        };

        let request = oauth::TokenRequest::new(
            authorization_code,
            self.conf.oauth_client_identifier.clone(),
            self.conf.oauth_client_secret.clone(),
            oauth::RedirectUri::PostMessage,
            oauth::GrantType::AuthorizationCode,
        );
        use hyper::Uri;
        let token_endpoint = "https://oauth2.googleapis.com/token"
            .parse::<Uri>()
            .unwrap();

        let tokenset = match request.exchange_code(token_endpoint).await {
            Ok(val) => val,
            Err(e) => {
                println!("{:?}", e);
                return Err(Error::InternalError);
            }
        };

        let id_token = match oauth::IdToken::new(&tokenset.id_token) {
            Ok(val) => val,
            Err(_) => return Err(Error::InternalError),
        };

        match users
            .filter(sub.eq(&id_token.sub))
            .get_result::<models::User>(&self.get_db())
        {
            Ok(val) => {
                if !val.active {
                    log::info!(
                        "Rejected authentication for deactivated account \"{}\"",
                        &id_token.email
                    );
                    return Err(Error::InvalidInput);
                }
            }
            Err(diesel::result::Error::NotFound) => {
                log::info!(
                    "Starting authentication for new account \"{}\"",
                    &id_token.email
                );
                if tokenset.refresh_token.is_none() {
                    log::info!("Failed to start authentication (missing refresh token) for new account \"{}\"", &id_token.email);
                    return Err(Error::InvalidInput);
                }
            }
            Err(_) => {
                log::error!(
                    "Failed to fetch information for account \"{}\"",
                    &id_token.email
                );
                return Err(Error::InternalError);
            }
        }

        let user = models::UserAddUpdate {
            sub: id_token.sub.clone(),
            email: id_token.email.clone(),
            given_name: id_token.given_name.clone(),
            family_name: id_token.family_name.clone(),
            picture: id_token.picture.clone(),
            oauth_access_token: tokenset.access_token.clone(),
            oauth_access_token_valid: Utc::now().naive_utc()
                + Duration::seconds(tokenset.expires_in.into()),
            oauth_refresh_token: tokenset.refresh_token,
            active: true,
        };

        let user = match diesel::update(users.filter(sub.eq(&user.sub)))
            .set(&user)
            .get_result::<models::User>(&self.get_db())
        {
            Ok(val) => {
                log::info!("Successfully updated account \"{}\"", &id_token.email);
                val
            }
            Err(diesel::result::Error::NotFound) => {
                match diesel::insert_into(users)
                    .values(&user)
                    .get_result::<models::User>(&self.get_db())
                {
                    Ok(val) => {
                        log::info!("Successfully created account \"{}\"", &id_token.email);
                        val
                    }
                    Err(_) => {
                        log::error!("Failed to create account \"{}\"", &id_token.email);
                        return Err(Error::InternalError);
                    }
                }
            }
            Err(_) => {
                log::error!("Failed to create account \"{}\"", &id_token.email);
                return Err(Error::InternalError);
            }
        };

        Ok(SessionToken {
            token: Jwt::new(
                user.id as u32,
                user.given_name,
                user.family_name,
                user.picture,
                3600,
            )
            .encode(&self.conf.jwt_secret()),
        })
    }

    /// Returns the validity and content of a session token
    async fn session_info(self, _: context::Context, token: String) -> RpcResult<SessionInfo> {
        match Jwt::decode(&self.conf.jwt_secret(), &token) {
            Ok(val) => Ok(SessionInfo {
                sub: val.sub,
                given_name: val.given_name,
                family_name: val.family_name,
                picture: val.picture,
                iat: val.iat,
                exp: val.exp,
            }),
            Err(_) => Err(Error::InvalidData),
        }
    }
}
