use std::env::{set_var, var};
use std::sync::Arc;

use chrono::{Duration, Utc};
use diesel::prelude::*;
use tarpc::context;
use uuid::Uuid;

use helpers::rpc::Error;
use identity::db::schema::{roles, users::dsl::users};
use identity::db::{get_db_pool, models, queries, DbPool};
use identity::rpc::models::{Role, SessionInfo, User};
use identity::rpc::{get_rpc_client, get_rpc_server, service::IdentityServiceClient};
use identity::{config::Configuration, session::jwt::Jwt};

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

        let user_role: models::Role = roles::dsl::roles
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
    set_var("SERVICE_SOCKET", "127.0.0.1:0");
    if var("OAUTH_CLIENT_IDENTIFIER").is_err() {
        set_var("OAUTH_CLIENT_IDENTIFIER", "test_oauth_client_identifier");
    }
    if var("OAUTH_CLIENT_SECRET").is_err() {
        set_var("OAUTH_CLIENT_SECRET", "test_oauth_client_secret");
    }
    if var("JWT_SECRET").is_err() {
        set_var("JWT_SECRET", "test_jwt_secret");
    }

    Configuration::init().unwrap()
}

async fn setup(
    test_context_name: String,
) -> Result<
    (
        impl futures::Future<Output = ()>,
        IdentityServiceClient,
        Arc<Configuration>,
        Arc<DbPool>,
        DbTestContext,
    ),
    (),
> {
    let configuration = Arc::new(get_test_configuration());
    let db_test_context = DbTestContext::new(
        configuration.get_db_connection_base_url(),
        test_context_name,
    );
    let db = Arc::new(get_db_pool(&db_test_context.get_connection_url()));
    let (server, socket) = get_rpc_server(
        configuration.get_service_socket(),
        configuration.clone(),
        db.clone(),
    )
    .await
    .unwrap();
    let client = get_rpc_client(socket).await.unwrap();

    Ok((server, client, configuration, db, db_test_context))
}

// get a valid user
#[tokio::test]
async fn get_user_exists() {
    // Arrange
    let (server, client, _configuration, db_pool, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    let user_role = queries::get_role_by_name("User", &db_pool.get().unwrap()).unwrap();

    let expected_result = User {
        id: Uuid::parse_str("a930312e-eb70-41e4-bf74-d88bf661d4dd").unwrap(),
        sub: "2".into(),
        email: "jack.kerr@example.net".into(),
        given_name: "Jack".into(),
        family_name: "Kerr".into(),
        picture: "https://example.com/avatar.jpg".into(),
        active: true,
        role_id: user_role.id,
    };

    // Act
    let result = client
        .get_user(
            context::current(),
            Uuid::parse_str("a930312e-eb70-41e4-bf74-d88bf661d4dd").unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(Ok(expected_result), result);
}

// get an invalid user
#[tokio::test]
async fn get_user_not_exists() {
    // Arrange
    let (server, client, _configuration, _db_pool, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    // Act
    let result = client
        .get_user(context::current(), Uuid::new_v4())
        .await
        .unwrap();

    // Assert
    assert_eq!(Err(Error::NotFound), result);
}

// list users with offset/limit
#[tokio::test]
async fn list_users_exists() {
    // Arrange
    let (server, client, _configuration, db_pool, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    let user_role = queries::get_role_by_name("User", &db_pool.get().unwrap()).unwrap();

    let expected_result = vec![
        User {
            id: Uuid::parse_str("42cf1a7b-b7ca-4baf-9dfa-41f4f454a7cf").unwrap(),
            sub: "3".into(),
            email: "justin.wilkins@example.net".into(),
            given_name: "Justin".into(),
            family_name: "Wilkins".into(),
            picture: "https://example.com/avatar.jpg".into(),
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
            active: true,
            role_id: user_role.id,
        },
    ];

    // Act
    let result = client
        .list_users(context::current(), 2, 2, Some(true))
        .await
        .unwrap();

    // Assert
    assert_eq!(Ok(expected_result), result);
}

// verify update user
#[tokio::test]
async fn update_user_verify() {
    // Arrange
    let (server, client, _configuration, db_pool, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    let user_role = queries::get_role_by_name("User", &db_pool.get().unwrap()).unwrap();

    let expected_result = User {
        id: Uuid::parse_str("a930312e-eb70-41e4-bf74-d88bf661d4dd").unwrap(),
        sub: "2".into(),
        email: "jack.kerr@example.net".into(),
        given_name: "Jack".into(),
        family_name: "Kerr".into(),
        picture: "https://example.com/avatar.jpg".into(),
        active: false,
        role_id: user_role.id,
    };

    // Act
    let result = client
        .update_user(context::current(), expected_result.clone())
        .await
        .unwrap();

    // Assert
    assert_eq!(Ok(expected_result), result);
}

// get a valid role
#[tokio::test]
async fn get_role_exists() {
    // Arrange
    let (server, client, _configuration, db_pool, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    let manager_role = queries::get_role_by_name("Manager", &db_pool.get().unwrap()).unwrap();

    let expected_result = Role {
        id: manager_role.id,
        name: "Manager".into(),
    };

    // Act
    let result = client
        .get_role(context::current(), manager_role.id)
        .await
        .unwrap();

    // Assert
    assert_eq!(Ok(expected_result), result);
}

// get an invalid role
#[tokio::test]
async fn get_role_not_exists() {
    // Arrange
    let (server, client, _configuration, _db_pool, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    // Act
    let result = client
        .get_role(context::current(), Uuid::new_v4())
        .await
        .unwrap();

    // Assert
    assert_eq!(Err(Error::NotFound), result);
}

// list roles with offset/limit
#[tokio::test]
async fn list_roles_exists() {
    // Arrange
    let (server, client, _configuration, db_pool, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

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
    let result = client.list_roles(context::current(), 1, 2).await.unwrap();

    // Assert
    assert_eq!(Ok(expected_result), result);
}

// test oauth authentication
#[tokio::test]
async fn oauth_authentication() {
    // Check if test requirements are met
    // This test will silently fail if the envrionment variable "OAUTH_AUTHORIZATION_CODE" is not set
    let authorization_code = match var("OAUTH_AUTHORIZATION_CODE") {
        Ok(val) => val,
        Err(_) => return,
    };

    // Arrange
    let (server, client, _configuration, _db_pool, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    // Act
    let result = client
        .oauth_authentication(context::current(), authorization_code)
        .await
        .unwrap();

    // Assert
    assert!(result.is_ok());
}

// test client identifier
#[tokio::test]
async fn oauth_client_identifier() {
    // Arrange
    let (server, client, _configuration, _db_pool, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    // Act
    let result = client
        .oauth_client_identifier(context::current())
        .await
        .unwrap()
        .unwrap();

    // Assert
    assert_eq!(var("OAUTH_CLIENT_IDENTIFIER").unwrap(), result.identifier);
}

// test session info with valid jwt signature
#[tokio::test]
async fn session_info_valid_token() {
    // Arrange
    let (server, client, configuration, _db_pool, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    let token = Jwt::new(
        Uuid::new_v4(),
        "John".into(),
        "Doe".into(),
        "https://example.com/avatar.jpg".into(),
        Utc::now(),
        Duration::seconds(3600),
    );

    let result = client
        .session_info(
            context::current(),
            token.encode(&configuration.get_jwt_secret()),
        )
        .await
        .unwrap();

    // Act
    let token = SessionInfo {
        sub: token.sub,
        given_name: token.given_name.clone(),
        family_name: token.family_name.clone(),
        picture: token.picture.clone(),
        iat: token.iat,
        exp: token.exp,
    };

    // Assert
    assert_eq!(Ok(token), result);
}

// test session info with invalid jwt signature
#[tokio::test]
async fn session_info_invalid_token() {
    // Arrange
    let (server, client, configuration, _db_pool, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    let token = Jwt::new(
        Uuid::new_v4(),
        "John".into(),
        "Doe".into(),
        "https://example.com/person.jpg".into(),
        Utc::now(),
        Duration::seconds(3600),
    );

    // Act
    let result = client
        .session_info(
            context::current(),
            token.encode(&(configuration.get_jwt_secret() + "abc")),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(Err(Error::InvalidData), result);
}
