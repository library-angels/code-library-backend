use std::env::set_var;
use std::sync::Arc;

use chrono::NaiveDate;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{prelude::*, result::Error};
use envconfig::Envconfig;

use identity::config::Configuration;
use identity::db::schema::{users::dsl::users, users_roles::dsl::users_roles};
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

        // run migration
        embedded_migrations::run(&conn).expect("Failed to apply database migration");

        // insert sample data for tests
        diesel::insert_into(users)
            .values(&sample_data::users())
            .execute(&conn)
            .expect("Error inserting 'users' sample data");

        diesel::insert_into(users_roles)
            .values(&sample_data::users_roles())
            .execute(&conn)
            .expect("Error inserting 'users_roles' sample data");

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

    let expected_result = User {
        id: 2,
        sub: "2".into(),
        email: "jack.kerr@example.net".into(),
        given_name: "Jack".into(),
        family_name: "Kerr".into(),
        picture: "https://example.com/avatar.jpg".into(),
        oauth_access_token: "access_token".into(),
        oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
        oauth_refresh_token: "refresh_token".into(),
        active: true,
    };

    // Act
    let result = queries::get_user(2, &db_pool.get().unwrap());

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
    let result = queries::get_user(20, &db_pool.get().unwrap());

    // Assert
    assert_eq!(Err(Error::NotFound), result);
}

// list active users with offset/limit
#[tokio::test]
async fn db_list_users_exists_active() {
    // Arrange
    let (_configuration, db_pool, _db_test_context) = setup(stdext::function_name!().into())
        .await
        .expect("Could not set up test environment");

    let expected_result = vec![
        User {
            id: 3,
            sub: "3".into(),
            email: "justin.wilkins@example.net".into(),
            given_name: "Justin".into(),
            family_name: "Wilkins".into(),
            picture: "https://example.com/avatar.jpg".into(),
            oauth_access_token: "access_token".into(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
            oauth_refresh_token: "refresh_token".into(),
            active: true,
        },
        User {
            id: 5,
            sub: "5".into(),
            email: "richard.henderson@example.net".into(),
            given_name: "Richard".into(),
            family_name: "Henderson".into(),
            picture: "https://example.com/avatar.jpg".into(),
            oauth_access_token: "access_token".into(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
            oauth_refresh_token: "refresh_token".into(),
            active: true,
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

    let expected_result = vec![User {
        id: 4,
        sub: "4".into(),
        email: "tim.jackson@example.net".into(),
        given_name: "Tim".into(),
        family_name: "Jackson".into(),
        picture: "https://example.com/avatar.jpg".into(),
        oauth_access_token: "access_token".into(),
        oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
        oauth_refresh_token: "refresh_token".into(),
        active: false,
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

    let expected_result = User {
        id: 2,
        sub: "2".into(),
        email: "jack.kerr@example.net".into(),
        given_name: "Jack".into(),
        family_name: "Kerr".into(),
        picture: "https://example.com/avatar.jpg".into(),
        oauth_access_token: "access_token".into(),
        oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
        oauth_refresh_token: "refresh_token".into(),
        active: false,
    };

    // Act
    let result = queries::update_user(expected_result.clone(), &db_pool.get().unwrap());

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

    let expected_result = Role {
        id: 2,
        name: "Manager".into(),
        access_manage_books: true,
        access_manage_roles: false,
    };

    // Act
    let result = queries::get_role(2, &db_pool.get().unwrap());

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
    let result = queries::get_role(20, &db_pool.get().unwrap());

    // Assert
    assert_eq!(Err(Error::NotFound), result);
}

// list roles with offset/limit
#[tokio::test]
async fn list_roles_exists() {
    // Arrange
    let (_configuration, db_pool, _db_test_context) = setup(stdext::function_name!().into())
        .await
        .expect("Could not set up test environment");

    let expected_result = vec![
        Role {
            id: 2,
            name: "Manager".into(),
            access_manage_books: true,
            access_manage_roles: false,
        },
        Role {
            id: 3,
            name: "Administrator".into(),
            access_manage_books: true,
            access_manage_roles: true,
        },
    ];

    // Act
    let result = queries::list_roles(1, 2, &db_pool.get().unwrap());

    // Assert
    assert_eq!(Ok(expected_result), result);
}

// update user role
#[tokio::test]
async fn db_update_user_role_verify() {
    // Arrange
    let (_configuration, db_pool, _db_test_context) = setup(stdext::function_name!().into())
        .await
        .expect("Could not set up test environment");

    let expected_result = UserRole {
        id: 3,
        user_id: 3,
        role_id: 3,
    };

    // Act
    let result = queries::update_user_role(expected_result.clone(), &db_pool.get().unwrap());

    // Assert
    assert_eq!(Ok(expected_result), result);
}
