use std::time::SystemTime;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub sub: String,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub oauth_access_token: String,
    pub oauth_access_token_valid: SystemTime,
    pub oauth_refresh_token: String,
    pub active: bool
}

#[derive(Queryable)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub access_manage_books: bool,
    pub access_manage_roles: bool
}
