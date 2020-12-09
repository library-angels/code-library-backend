use chrono::{NaiveDateTime, Utc};

use super::schema::*;
use crate::rpc::models as RpcModels;

#[derive(Clone, Debug, PartialEq, Queryable)]
pub struct User {
    pub id: i32,
    pub sub: String,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub oauth_access_token: String,
    pub oauth_access_token_valid: NaiveDateTime,
    pub oauth_refresh_token: String,
    pub active: bool,
    pub role_id: i32,
}

impl From<RpcModels::User> for User {
    fn from(user: RpcModels::User) -> Self {
        User {
            id: user.id,
            sub: user.sub,
            email: user.email,
            given_name: user.given_name,
            family_name: user.family_name,
            picture: user.picture,
            oauth_access_token: "".into(),
            oauth_access_token_valid: Utc::now().naive_utc(),
            oauth_refresh_token: "".into(),
            active: user.active,
            role_id: user.role_id,
        }
    }
}

#[derive(Clone, Debug, AsChangeset, Insertable, PartialEq)]
#[table_name = "users"]
pub struct UserAddUpdate {
    pub sub: String,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub oauth_access_token: String,
    pub oauth_access_token_valid: NaiveDateTime,
    pub oauth_refresh_token: Option<String>,
    pub active: bool,
    pub role_id: i32,
}

#[derive(Clone, Debug, PartialEq, Queryable)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub access_manage_books: bool,
    pub access_manage_roles: bool,
}
