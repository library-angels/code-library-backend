use std::net::SocketAddr;
use tarpc::context;
use super::service::*;
use crate::db::models;
use crate::DB;
use diesel::prelude::*;

#[derive(Clone)]
pub struct IdentityService(pub SocketAddr);

#[tarpc::server]
impl Identity for IdentityService {
    /// Returns an user
    async fn user(self, _: context::Context, user_id: u32) -> Result<User, Error> {
        use crate::db::schema::users::dsl::*;

        let result = users.find(user_id as i32).first::<models::User>(&DB.get().unwrap().get().unwrap());

        match result {
            Ok(val) => Ok(
                User {
                    id: val.id,
                    sub: val.sub,
                    email: val.email,
                    given_name: val.given_name,
                    family_name: val.family_name,
                    picture: val.picture,
                    active: val.active
                }
            ),
            Err(diesel::result::Error::NotFound) => Err(Error::NotFound),
            Err(_) => Err(Error::InternalError)
        }
    }

    /// Returns a list of users
    async fn users(
        self,
        _: context::Context,
        offset: u32,
        limit: u32,
        user_active: Option<bool>,
    ) -> Result<Vec<User>, Error> {
        use crate::db::schema::users::dsl::*;

        let results = match user_active {
            Some(val) => users.filter(active.eq(val)).offset(offset as i64).limit(limit as i64).load::<models::User>(&DB.get().unwrap().get().unwrap()),
            None => users.offset(offset as i64).limit(limit as i64).load::<models::User>(&DB.get().unwrap().get().unwrap()),
        };

        match results {
            Ok(val) => Ok(
                val.iter().map(|x|
                    User {
                        id: x.id,
                        sub: x.sub.clone(),
                        email: x.email.clone(),
                        given_name: x.given_name.clone(),
                        family_name: x.family_name.clone(),
                        picture: x.picture.clone(),
                        active: x.active
                    }
                ).collect::<Vec<User>>()
            ),
            Err(diesel::result::Error::NotFound) => Err(Error::NotFound),
            Err(_) => Err(Error::InternalError)
        }
    }

    /// Returns a role
    async fn role(self, _: context::Context, role_id: u32) -> Result<Role, Error> {
        use crate::db::schema::roles::dsl::*;

        let result = roles.find(role_id as i32).first::<models::Role>(&DB.get().unwrap().get().unwrap());

        match result {
            Ok(val) => Ok(
                Role {
                    id: val.id,
                    name: val.name,
                    access_manage_books: val.access_manage_books,
                    access_manage_roles: val.access_manage_roles
                }
            ),
            Err(diesel::result::Error::NotFound) => Err(Error::NotFound),
            Err(_) => Err(Error::InternalError)
        }
    }

    /// Returns a list of roles
    async fn roles(
        self,
        _: context::Context,
        offset: u32,
        limit: u32,
    ) -> Result<Vec<Role>, Error> {
        use crate::db::schema::roles::dsl::*;

        let results = roles.offset(offset as i64).limit(limit as i64).load::<models::Role>(&DB.get().unwrap().get().unwrap());


        match results {
            Ok(val) => Ok(
                val.iter().map(|x|
                    Role {
                        id: x.id,
                        name: x.name.clone(),
                        access_manage_books: x.access_manage_books,
                        access_manage_roles: x.access_manage_roles
                    }
                ).collect::<Vec<Role>>()
            ),
            Err(diesel::result::Error::NotFound) => Err(Error::NotFound),
            Err(_) => Err(Error::InternalError)
        }
    }

    /// Returns an user role
    async fn user_role(self, _: context::Context, user_role_id: u32) -> Result<UserRole, Error> {
        use crate::db::schema::users_roles::dsl::*;

        let result = users_roles.find(user_role_id as i32).first::<models::UserRole>(&DB.get().unwrap().get().unwrap());

        match result {
            Ok(val) => Ok(
                UserRole {
                    id: val.id,
                    user_id: val.user_id,
                    role_id: val.role_id
                }
            ),
            Err(diesel::result::Error::NotFound) => Err(Error::NotFound),
            Err(_) => Err(Error::InternalError)
        }
    }

    /// Returns a list of users
    async fn user_roles(
        self,
        _: context::Context,
        offset: u32,
        limit: u32,
        role_id: Option<u32>
    ) -> Result<Vec<UserRole>, Error> {
        use crate::db::schema::users_roles::dsl;

        let results = match role_id {
            Some(val) => dsl::users_roles.filter(dsl::role_id.eq(val as i32)).offset(offset as i64).limit(limit as i64).load::<models::UserRole>(&DB.get().unwrap().get().unwrap()),
            None => dsl::users_roles.offset(offset as i64).limit(limit as i64).load::<models::UserRole>(&DB.get().unwrap().get().unwrap()),
        };

        match results {
            Ok(val) => Ok(
                val.iter().map(|x|
                    UserRole {
                        id: x.id,
                        user_id: x.user_id,
                        role_id: x.role_id
                    }
                ).collect::<Vec<UserRole>>()
            ),
            Err(diesel::result::Error::NotFound) => Err(Error::NotFound),
            Err(_) => Err(Error::InternalError)
        }
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
