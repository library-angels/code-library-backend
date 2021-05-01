//! These tests need the feature "test-data" being enabled in order to run

use std::env;
use tarpc::context;

#[macro_use]
extern crate diesel_migrations;

use book::{models::Error, test_data};
use helpers::result::StdResult;

#[tokio::test]
async fn should_not_find_book() -> StdResult<()> {
    // Arrange
    let client = utils::test_setup().await?;

    // Act
    let book_not_found = client.get_book(context::current(), 4).await?;

    // Assert
    assert_eq!(
        book_not_found.unwrap_err(),
        Error::NotFound,
        "Database should be empty"
    );
    Ok(())
}

#[tokio::test]
async fn should_find_book() -> StdResult<()> {
    // Arrange
    let client = utils::test_setup().await?;

    // Act
    let found_book = client.get_book(context::current(), 1).await?;

    // Assert
    assert_eq!(
        found_book.unwrap(),
        test_data::book_1(),
        "Database should be filled"
    );
    Ok(())
}

mod utils {
    use std::{io, net::SocketAddr};

    use diesel::prelude::*;
    use uuid::Uuid;

    use book::{
        db::{self, DbPool},
        rpc_client, rpc_server,
        service::BookServiceClient,
    };

    use super::*;

    pub async fn test_setup() -> StdResult<BookServiceClient> {
        // database
        let db_pool = db_setup()?;
        test_data::insert_data(&db_pool.get()?)?;
        // rpc
        let addr = spawn_rpc_server(db_pool).await?;
        let client = rpc_client(&addr).await?;

        Ok(client)
    }

    fn db_setup() -> StdResult<DbPool> {
        // create dummy database
        let tmp_conn = PgConnection::establish(&format!(
            "postgres://postgres:password@{}",
            env::var("DB_HOST_SOCKET").unwrap()
        ))?;
        let db_name = Uuid::new_v4();
        diesel::sql_query(&*format!("CREATE DATABASE \"{}\";", db_name)).execute(&tmp_conn)?;

        // connect to dummy database
        let database_url = format!(
            "{}/{}",
            &format!(
                "postgres://postgres:password@{}",
                env::var("DB_HOST_SOCKET").unwrap()
            ),
            db_name
        );
        let db_pool = db::get_db_pool(&*database_url);

        // migrate dummy database
        embed_migrations!();
        helpers::db::run_migration(embedded_migrations::run, &db_pool);

        Ok(db_pool)
    }

    async fn spawn_rpc_server(db_pool: DbPool) -> io::Result<SocketAddr> {
        let (server, addr) = rpc_server(&SocketAddr::from(([127, 0, 0, 1], 0)), db_pool).await?;
        tokio::spawn(server);
        Ok(addr)
    }
}
