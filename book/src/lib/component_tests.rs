//! Component tests (run with mocked queries)

use tarpc::context;

use helpers::result::StdResult;

use crate::{models::Error, test_data};

#[tokio::test]
async fn should_find_book() -> StdResult<()> {
    // Arrange
    let mut client = utils::test_setup().await?;

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

#[tokio::test]
async fn should_not_find_book() -> StdResult<()> {
    // Arrange
    let mut client = utils::test_setup().await?;

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

mod utils {
    use std::{io, net::SocketAddr};

    use crate::{
        db::{self, DbPool},
        rpc_client, rpc_server,
        service::BookServiceClient,
    };

    pub async fn test_setup() -> io::Result<BookServiceClient> {
        // database
        let db_pool = db::get_db_pool("postgres://postgres:password@localhost");
        // rpc
        let addr = spawn_rpc_server(db_pool).await?;
        let client = rpc_client(&addr).await?;

        Ok(client)
    }

    async fn spawn_rpc_server(db_pool: DbPool) -> io::Result<SocketAddr> {
        let (server, addr) = rpc_server(&SocketAddr::from(([127, 0, 0, 1], 0)), db_pool).await?;
        tokio::spawn(server);
        Ok(addr)
    }
}
