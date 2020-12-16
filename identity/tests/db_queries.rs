use std::env::set_var;
use std::sync::Arc;

use chrono::NaiveDate;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{prelude::*, result::Error};
use envconfig::Envconfig;
use uuid::Uuid;

use identity::config::Configuration;
use identity::db::schema::{roles, users::dsl::users};
use identity::db::{get_db_pool, models::*, queries};

mod sample_data;

#[macro_use]
extern crate diesel_migrations;

embed_migrations!("migrations/");

struct DbTestContext {
    connection_url: String,
    db_name: String,
}

impl DbTestContext {
    fn new(connection_url: String, db_name: String) -> Self {
        // connect to database service
        let conn = PgConnection::establish(&connection_url)
            .expect("Could not connect to database service");

        // create new database
        diesel::sql_query(format!("CREATE DATABASE \"{}\"", db_name))
            .execute(&conn)
            .unwrap_or_else(|_| panic!("Could not create database: {}", db_name));

        // reconnect to the new database
        let conn = PgConnection::establish(&format!("{}/{}", connection_url, db_name))
            .expect("Could not connect to database service");

        // create uuid extension on database
        diesel::sql_query("CREATE EXTENSION \"uuid-ossp\"")
            .execute(&conn)
            .unwrap_or_else(|_| panic!("Could not create extension"));

        // run migration
        embedded_migrations::run(&conn).expect("Failed to apply database migration");

        let user_role: Role = roles::dsl::roles
            .filter(roles::dsl::name.eq("User"))
            .get_result(&conn)
            .unwrap();

        // insert sample data for tests
        diesel::insert_into(users)
            .values(&sample_data::users(user_role.id))
            .execute(&conn)
            .expect("Error inserting 'users' sample data");

        Self {
            connection_url,
            db_name,
        }
    }

    fn get_connection_url(&self) -> String {
        format!("{}/{}", self.connection_url, self.db_name)
    }
}

impl Drop for DbTestContext {
    fn drop(&mut self) {
        let conn = PgConnection::establish(&self.connection_url)
            .expect("Could not connect to database service");

        diesel::sql_query(format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}'",
            self.db_name
        ))
        .execute(&conn)
        .unwrap();

        diesel::sql_query(format!("DROP DATABASE \"{}\"", self.db_name))
            .execute(&conn)
            .unwrap_or_else(|_| panic!("Could not drop database: {}", self.db_name));
    }
}

fn get_test_configuration() -> Configuration {
    set_var("OAUTH_CLIENT_IDENTIFIER", "test_oauth_client_identifier");
    set_var("OAUTH_CLIENT_SECRET", "test_oauth_client_secret");
    set_var("JWT_SECRET", "test_jwt_secret");

    Configuration::init_from_env().unwrap()
}

async fn setup(
    test_context_name: String,
) -> Result<
    (
        Arc<Configuration>,
        Arc<Pool<ConnectionManager<PgConnection>>>,
        DbTestContext,
    ),
    (),
> {
    let configuration = Arc::new(get_test_configuration());
    let db_test_context =
        DbTestContext::new(configuration.db_connection_base_url(), test_context_name);
    let db_pool = Arc::new(get_db_pool(&db_test_context.get_connection_url()));

    Ok((configuration, db_pool, db_test_context))
}

// get a valid user
#[tokio::test]
async fn db_get_user_exists() {
    // Arrange
    let (_configuration, db_pool, _db_test_context) = setup(stdext::function_name!().into())
        .await
        .expect("Could not set up test environment");

    let user_role = queries::get_role_by_name("User", &db_pool.get().unwrap()).unwrap();

    let expected_result = User {
        id: Uuid::parse_str("a930312e-eb70-41e4-bf74-d88bf661d4dd").unwrap(),
        sub: "2".into(),
        email: "jack.kerr@example.net".into(),
        given_name: "Jack".into(),
        family_name: "Kerr".into(),
        picture: "https://example.com/avatar.jpg".into(),
        oauth_access_token: "access_token".into(),
        oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
        oauth_refresh_token: "refresh_token".into(),
        active: true,
        role_id: user_role.id,
    };

    // Act
    let result = queries::get_user(
        Uuid::parse_str("a930312e-eb70-41e4-bf74-d88bf661d4dd").unwrap(),
        &db_pool.get().unwrap(),
    );

    // Assert
    assert_eq!(Ok(expected_result), result);
}

// get an invalid user
#[tokio::test]
async fn db_get_user_not_exists() {
    // Arrange
    let (_configuration, db_pool, _db_test_context) = setup(stdext::function_name!().into())
        .await
        .expect("Could not set up test environment");

    // Act
    let result = queries::get_user(Uuid::new_v4(), &db_pool.get().unwrap());

    // Assert
    assert_eq!(Err(Error::NotFound), result);
}

// get an user by its sub
#[tokio::test]
async fn db_get_user_by_sub() {
    // Arrange
    let (_configuration, db_pool, _db_test_context) = setup(stdext::function_name!().into())
        .await
        .expect("Could not set up test environment");

    let user_role = queries::get_role_by_name("User", &db_pool.get().unwrap()).unwrap();

    let expected_result = User {
        id: Uuid::parse_str("a930312e-eb70-41e4-bf74-d88bf661d4dd").unwrap(),
        sub: "2".into(),
        email: "jack.kerr@example.net".into(),
        given_name: "Jack".into(),
        family_name: "Kerr".into(),
        picture: "https://example.com/avatar.jpg".into(),
        oauth_access_token: "access_token".into(),
        oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
        oauth_refresh_token: "refresh_token".into(),
        active: true,
        role_id: user_role.id,
    };

    // Act
    let result = queries::get_user_by_sub("2", &db_pool.get().unwrap());

    // Assert
    assert_eq!(Ok(expected_result), result);
}

// list active users with offset/limit
#[tokio::test]
async fn db_list_users_exists_active() {
    // Arrange
    let (_configuration, db_pool, _db_test_context) = setup(stdext::function_name!().into())
        .await
        .expect("Could not set up test environment");

    let user_role = queries::get_role_by_name("User", &db_pool.get().unwrap()).unwrap();

    let expected_result = vec![
        User {
            id: Uuid::parse_str("42cf1a7b-b7ca-4baf-9dfa-41f4f454a7cf").unwrap(),
            sub: "3".into(),
            email: "justin.wilkins@example.net".into(),
            given_name: "Justin".into(),
            family_name: "Wilkins".into(),
            picture: "https://example.com/avatar.jpg".into(),
            oauth_access_token: "access_token".into(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
            oauth_refresh_token: "refresh_token".into(),
            active: true,
            role_id: user_role.id,
        },
        User {
            id: Uuid::parse_str("7cae5854-490c-4ee0-970b-8ec8a77c7c7a").unwrap(),
            sub: "5".into(),
            email: "richard.henderson@example.net".into(),
            given_name: "Richard".into(),
            family_name: "Henderson".into(),
            picture: "https://example.com/avatar.jpg".into(),
            oauth_access_token: "access_token".into(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
            oauth_refresh_token: "refresh_token".into(),
            active: true,
            role_id: user_role.id,
        },
    ];

    // Act
    let result = queries::list_users(2, 2, Some(true), &db_pool.get().unwrap());

    // Assert
    assert_eq!(Ok(expected_result), result);
}

// list inactive users with offset/limit
#[tokio::test]
async fn db_list_users_exists_inactive() {
    // Arrange
    let (_configuration, db_pool, _db_test_context) = setup(stdext::function_name!().into())
        .await
        .expect("Could not set up test environment");

    let user_role = queries::get_role_by_name("User", &db_pool.get().unwrap()).unwrap();

    let expected_result = vec![User {
        id: Uuid::parse_str("ad996820-085d-4b02-8a3d-10f0527a1ba0").unwrap(),
        sub: "4".into(),
        email: "tim.jackson@example.net".into(),
        given_name: "Tim".into(),
        family_name: "Jackson".into(),
        picture: "https://example.com/avatar.jpg".into(),
        oauth_access_token: "access_token".into(),
        oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
        oauth_refresh_token: "refresh_token".into(),
        active: false,
        role_id: user_role.id,
    }];

    // Act
    let result = queries::list_users(0, 1, Some(false), &db_pool.get().unwrap());

    // Assert
    assert_eq!(Ok(expected_result), result);
}

// verify update user
#[tokio::test]
async fn db_update_user_verify() {
    // Arrange
    let (_configuration, db_pool, _db_test_context) = setup(stdext::function_name!().into())
        .await
        .expect("Could not set up test environment");

    let user_role = queries::get_role_by_name("User", &db_pool.get().unwrap()).unwrap();

    let expected_result = User {
        id: Uuid::parse_str("a930312e-eb70-41e4-bf74-d88bf661d4dd").unwrap(),
        sub: "2".into(),
        email: "jack.kerr@example.net".into(),
        given_name: "Jack".into(),
        family_name: "Kerr".into(),
        picture: "https://example.com/avatar.jpg".into(),
        oauth_access_token: "access_token".into(),
        oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
        oauth_refresh_token: "refresh_token".into(),
        active: false,
        role_id: user_role.id,
    };

    // Act
    let result = queries::update_user(expected_result.clone(), &db_pool.get().unwrap());

    // Assert
    assert_eq!(Ok(expected_result), result);
}

// verify update user by sub
#[tokio::test]
async fn db_update_user_by_sub_verify() {
    // Arrange
    let (_configuration, db_pool, _db_test_context) = setup(stdext::function_name!().into())
        .await
        .expect("Could not set up test environment");

    let user_role = queries::get_role_by_name("User", &db_pool.get().unwrap()).unwrap();

    let expected_result = User {
        id: Uuid::parse_str("a930312e-eb70-41e4-bf74-d88bf661d4dd").unwrap(),
        sub: "2".into(),
        email: "jack.kerr@example.net".into(),
        given_name: "Jack".into(),
        family_name: "Kerr".into(),
        picture: "https://example.com/avatar.jpg".into(),
        oauth_access_token: "access_token_new".into(),
        oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
        oauth_refresh_token: "refresh_token".into(),
        active: true,
        role_id: user_role.id,
    };

    let expected_change = UserAddUpdate {
        id: Uuid::parse_str("a930312e-eb70-41e4-bf74-d88bf661d4dd").unwrap(),
        sub: "2".into(),
        email: "jack.kerr@example.net".into(),
        given_name: "Jack".into(),
        family_name: "Kerr".into(),
        picture: "https://example.com/avatar.jpg".into(),
        oauth_access_token: "access_token_new".into(),
        oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
        oauth_refresh_token: None,
        active: true,
        role_id: user_role.id,
    };

    // Act
    let result = queries::update_user_by_sub(expected_change, &db_pool.get().unwrap());

    // Assert
    assert_eq!(Ok(expected_result), result);
}

// get a valid role
#[tokio::test]
async fn db_get_role_exists() {
    // Arrange
    let (_configuration, db_pool, _db_test_context) = setup(stdext::function_name!().into())
        .await
        .expect("Could not set up test environment");

    let manager_role = queries::get_role_by_name("Manager", &db_pool.get().unwrap()).unwrap();

    let expected_result = Role {
        id: manager_role.id,
        name: "Manager".into(),
    };

    // Act
    let result = queries::get_role(manager_role.id, &db_pool.get().unwrap());

    // Assert
    assert_eq!(Ok(expected_result), result);
}

// get an invalid role
#[tokio::test]
async fn db_get_role_not_exists() {
    // Arrange
    let (_configuration, db_pool, _db_test_context) = setup(stdext::function_name!().into())
        .await
        .expect("Could not set up test environment");

    // Act
    let result = queries::get_role(Uuid::new_v4(), &db_pool.get().unwrap());

    // Assert
    assert_eq!(Err(Error::NotFound), result);
}

// list roles with offset/limit
#[tokio::test]
async fn db_list_roles_exists() {
    // Arrange
    let (_configuration, db_pool, _db_test_context) = setup(stdext::function_name!().into())
        .await
        .expect("Could not set up test environment");

    let manager_role = queries::get_role_by_name("Manager", &db_pool.get().unwrap()).unwrap();
    let administrator_role =
        queries::get_role_by_name("Administrator", &db_pool.get().unwrap()).unwrap();

    let expected_result = vec![
        Role {
            id: manager_role.id,
            name: "Manager".into(),
        },
        Role {
            id: administrator_role.id,
            name: "Administrator".into(),
        },
    ];

    // Act
    let result = queries::list_roles(1, 2, &db_pool.get().unwrap());

    // Assert
    assert_eq!(Ok(expected_result), result);
}
