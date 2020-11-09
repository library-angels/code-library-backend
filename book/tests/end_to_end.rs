mod test_data;

use std::error::Error as StdError;

use tarpc::context;

#[macro_use]
extern crate diesel_migrations;

use book::models::Error;

type StdResult<T> = Result<T, Box<dyn StdError>>;

#[tokio::test]
async fn should_not_find_book() -> StdResult<()> {
    // Arrange
    let (_, mut client) = utils::test_setup().await?;

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
    let (_, mut client) = utils::test_setup().await?;

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
    use std::{net::SocketAddr, sync::Arc};

    use diesel::prelude::*;
    use uuid::Uuid;

    use book::db::{self, DbConn, DbPool};
    use book::service::BookServiceClient;
    use book::{rpc_client, rpc_server};

    use super::*;

    const DATABASE_URL: &str = "postgres://postgres:password@localhost";
    const RPC_SERVER_ADDR: &str = "127.0.0.1:0"; // get an ephemeral address

    pub async fn test_setup() -> StdResult<(DbConn, BookServiceClient)> {
        // database
        let db_pool = db_setup()?;
        let db_conn = db_pool.get()?;
        test_data::insert_data(&db_conn)?;
        // rpc
        let addr = spawn_rpc_server(db_pool).await?;
        let client = rpc_client(&addr).await?;

        Ok((db_conn, client))
    }

    fn db_setup() -> StdResult<DbPool> {
        // create dummy database
        let tmp_conn = PgConnection::establish(DATABASE_URL)?;
        let db_name = Uuid::new_v4();
        diesel::sql_query(&*format!("CREATE DATABASE \"{}\";", db_name)).execute(&tmp_conn)?;

        // connect to dummy database
        let database_url = format!("{}/{}", DATABASE_URL, db_name);
        let db_pool = db::new_db_pool(&*database_url);

        // migrate dummy database
        embed_migrations!();
        helpers::db::run_migration(embedded_migrations::run, &db_pool);

        Ok(db_pool)
    }

    async fn spawn_rpc_server(db_pool: DbPool) -> StdResult<SocketAddr> {
        let (server, addr) = rpc_server(&RPC_SERVER_ADDR.parse()?, Arc::new(db_pool)).await?;
        tokio::spawn(server);
        Ok(addr)
    }
}
