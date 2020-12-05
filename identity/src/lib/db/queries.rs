use diesel::prelude::*;
use diesel::result::QueryResult;

use super::models::*;
use super::schema;
use super::DbConn;

pub fn create_user(user: UserAddUpdate, db: &DbConn) -> QueryResult<User> {
    use schema::users::dsl::users;

    diesel::insert_into(users).values(&user).get_result(db)
}

pub fn get_user(user_id: i32, db: &DbConn) -> QueryResult<User> {
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

    diesel::update(users.find(user.id as i32))
        .set(active.eq(user.active))
        .get_result(db)
}

pub fn get_role(role_id: i32, db: &DbConn) -> QueryResult<Role> {
    use schema::roles::dsl::roles;

    roles.find(role_id).first(db)
}

pub fn list_roles(offset: i64, limit: i64, db: &DbConn) -> QueryResult<Vec<Role>> {
    use schema::roles::dsl::roles;

    roles.offset(offset).limit(limit).load(db)
}

pub fn get_user_role(user_role_id: i32, db: &DbConn) -> QueryResult<UserRole> {
    use schema::users_roles::dsl::users_roles;

    users_roles.find(user_role_id).first(db)
}

pub fn list_user_roles(
    offset: i64,
    limit: i64,
    role_id: Option<i32>,
    db: &DbConn,
) -> QueryResult<Vec<UserRole>> {
    use schema::users_roles::dsl;

    match role_id {
        Some(val) => dsl::users_roles
            .filter(dsl::role_id.eq(val))
            .offset(offset)
            .limit(limit)
            .load(db),
        None => dsl::users_roles.offset(offset).limit(limit).load(db),
    }
}

pub fn update_user_role(user_role: UserRole, db: &DbConn) -> QueryResult<UserRole> {
    use schema::users_roles::dsl::*;

    diesel::update(users_roles.find(user_role.id as i32))
        .set(role_id.eq(user_role.role_id as i32))
        .get_result(db)
}
