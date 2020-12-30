use diesel::prelude::*;
use diesel::result::QueryResult;
use uuid::Uuid;

use super::models::*;
use super::schema;
use super::DbConn;

pub fn create_user(user: UserAddUpdate, db: &DbConn) -> QueryResult<User> {
    use schema::users::dsl::users;

    diesel::insert_into(users).values(&user).get_result(db)
}

pub fn get_user(user_id: Uuid, db: &DbConn) -> QueryResult<User> {
    use schema::users::dsl::users;

    users.find(user_id).first(db)
}

pub fn get_user_by_sub(sub: &str, db: &DbConn) -> QueryResult<User> {
    use schema::users::dsl;

    dsl::users.filter(dsl::sub.eq(sub)).get_result(db)
}

pub fn list_users(
    offset: i64,
    limit: i64,
    user_active: Option<bool>,
    db: &DbConn,
) -> QueryResult<Vec<User>> {
    use schema::users::dsl::*;

    match user_active {
        Some(val) => users
            .filter(active.eq(val))
            .offset(offset)
            .limit(limit)
            .load(db),
        None => users.offset(offset).limit(limit).load(db),
    }
}

pub fn update_user(user: User, db: &DbConn) -> QueryResult<User> {
    use schema::users::dsl::*;

    diesel::update(users.find(user.id))
        .set(active.eq(user.active))
        .get_result(db)
}

pub fn update_user_by_sub(user: UserAddUpdate, db: &DbConn) -> QueryResult<User> {
    use schema::users::dsl::*;

    diesel::update(users.filter(sub.eq(&user.sub)))
        .set(&user)
        .get_result(db)
}

pub fn get_role(role_id: Uuid, db: &DbConn) -> QueryResult<Role> {
    use schema::roles::dsl::roles;

    roles.find(role_id).first(db)
}

pub fn get_role_by_name(role_name: &str, db: &DbConn) -> QueryResult<Role> {
    use schema::roles::dsl::*;

    roles.filter(name.eq(role_name)).get_result(db)
}

pub fn list_roles(offset: i64, limit: i64, db: &DbConn) -> QueryResult<Vec<Role>> {
    use schema::roles::dsl::roles;

    roles.offset(offset).limit(limit).load(db)
}
