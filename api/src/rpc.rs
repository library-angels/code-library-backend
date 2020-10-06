use tarpc::client;
use tokio_serde::formats::Json;

use crate::CONFIGURATION;

pub async fn identity_client() -> Result<identity_rpc_service::IdentityServiceClient, std::io::Error>
{
    #[allow(clippy::redundant_closure)]
    let transport = tarpc::serde_transport::tcp::connect(
        CONFIGURATION.get().unwrap().identity_service_socket(),
        || Json::default(),
    )
    .await;
    match transport {
        Ok(val) => {
            identity_rpc_service::IdentityServiceClient::new(client::Config::default(), val).spawn()
        }
        Err(e) => Err(e),
    }
}
