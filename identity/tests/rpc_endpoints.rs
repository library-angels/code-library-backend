use std::env::{set_var, var};
use std::sync::Arc;

use diesel::prelude::*;
use envconfig::Envconfig;
use tarpc::context;

use helpers::rpc::Error;
use identity::db::get_db_pool;
use identity::db::schema::{users::dsl::users, users_roles::dsl::users_roles};
use identity::rpc::models::{Role, SessionInfo, User, UserRole};
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
    set_var("RPC_HOST_PORT", "0");
    if var("OAUTH_CLIENT_IDENTIFIER").is_err() {
        set_var("OAUTH_CLIENT_IDENTIFIER", "test_oauth_client_identifier");
    }
    if var("OAUTH_CLIENT_SECRET").is_err() {
        set_var("OAUTH_CLIENT_SECRET", "test_oauth_client_secret");
    }
    if var("JWT_SECRET").is_err() {
        set_var("JWT_SECRET", "test_jwt_secret");
    }

    Configuration::init_from_env().unwrap()
}

async fn setup(
    test_context_name: String,
) -> Result<
    (
        impl futures::Future<Output = ()>,
        IdentityServiceClient,
        Arc<Configuration>,
        DbTestContext,
    ),
    (),
> {
    let configuration = Arc::new(get_test_configuration());
    let db_test_context =
        DbTestContext::new(configuration.db_connection_base_url(), test_context_name);
    let db = Arc::new(get_db_pool(&db_test_context.get_connection_url()));
    let (server, socket) = get_rpc_server(
        configuration.rpc_socket(),
        configuration.clone(),
        db.clone(),
    )
    .await
    .unwrap();
    let client = get_rpc_client(socket).await.unwrap();

    Ok((server, client, configuration, db_test_context))
}

// get a valid user
#[tokio::test]
async fn get_user_exists() {
    // Arrange
    let (server, mut client, _configuration, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    let expected_result = User {
        id: 2,
        sub: "2".into(),
        email: "jack.kerr@example.net".into(),
        given_name: "Jack".into(),
        family_name: "Kerr".into(),
        picture: "https://example.com/avatar.jpg".into(),
        active: true,
    };

    // Act
    let result = client.get_user(context::current(), 2).await.unwrap();

    // Assert
    assert_eq!(Ok(expected_result), result);
}

// get an invalid user
#[tokio::test]
async fn get_user_not_exists() {
    // Arrange
    let (server, mut client, _configuration, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    // Act
    let result = client.get_user(context::current(), 20).await.unwrap();

    // Assert
    assert_eq!(Err(Error::NotFound), result);
}

// list users with offset/limit
#[tokio::test]
async fn list_users_exists() {
    // Arrange
    let (server, mut client, _configuration, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    let expected_result = vec![
        User {
            id: 3,
            sub: "3".into(),
            email: "justin.wilkins@example.net".into(),
            given_name: "Justin".into(),
            family_name: "Wilkins".into(),
            picture: "https://example.com/avatar.jpg".into(),
            active: true,
        },
        User {
            id: 5,
            sub: "5".into(),
            email: "richard.henderson@example.net".into(),
            given_name: "Richard".into(),
            family_name: "Henderson".into(),
            picture: "https://example.com/avatar.jpg".into(),
            active: true,
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
    let (server, mut client, _configuration, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    let expected_result = User {
        id: 2,
        sub: "2".into(),
        email: "jack.kerr@example.net".into(),
        given_name: "Jack".into(),
        family_name: "Kerr".into(),
        picture: "https://example.com/avatar.jpg".into(),
        active: false,
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
    let (server, mut client, _configuration, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    let expected_result = Role {
        id: 2,
        name: "Manager".into(),
        access_manage_books: true,
        access_manage_roles: false,
    };

    // Act
    let result = client.get_role(context::current(), 2).await.unwrap();

    // Assert
    assert_eq!(Ok(expected_result), result);
}

// get an invalid role
#[tokio::test]
async fn get_role_not_exists() {
    // Arrange
    let (server, mut client, _configuration, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    // Act
    let result = client.get_role(context::current(), 20).await.unwrap();

    // Assert
    assert_eq!(Err(Error::NotFound), result);
}

// list roles with offset/limit
#[tokio::test]
async fn list_roles_exists() {
    // Arrange
    let (server, mut client, _configuration, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

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
    let result = client.list_roles(context::current(), 1, 2).await.unwrap();

    // Assert
    assert_eq!(Ok(expected_result), result);
}

// update user role
#[tokio::test]
async fn update_user_role_verify() {
    // Arrange
    let (server, mut client, _configuration, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    let expected_result = UserRole {
        id: 3,
        user_id: 3,
        role_id: 3,
    };

    // Act
    let result = client
        .update_user_role(context::current(), expected_result.clone())
        .await
        .unwrap();

    // Assert
    assert_eq!(Ok(expected_result), result);
}

/*
 * TODO: test oauth_authentication
 * Reason: Return of method is currently unpredictable, because it is based on time.
 */

// test client identifier
#[tokio::test]
async fn oauth_client_identifier() {
    // Arrange
    let (server, mut client, _configuration, _db_test_context) =
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
    let (server, mut client, configuration, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    let token = Jwt::new(
        1,
        "John".into(),
        "Doe".into(),
        "https://example.com/avatar.jpg".into(),
        3600,
    );

    let result = client
        .session_info(
            context::current(),
            token.encode(&configuration.jwt_secret()),
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
    let (server, mut client, configuration, _db_test_context) =
        setup(stdext::function_name!().into())
            .await
            .expect("Could not set up test environment");
    tokio::spawn(server);

    let token = Jwt::new(
        1,
        "John".into(),
        "Doe".into(),
        "https://example.com/person.jpg".into(),
        3600,
    );

    // Act
    let result = client
        .session_info(
            context::current(),
            token.encode(&(configuration.jwt_secret() + "abc")),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(Err(Error::InvalidData), result);
}
