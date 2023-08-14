use common_lib::blind_auth_api::blind_auth_server::BlindAuthServer;
use common_lib::store::store::DataStore;

use common_lib::verifier::AuthServer;
use log::info;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let addr = "0.0.0.0:50051".parse()?;
    let store = DataStore::new();
    let blind_auth = AuthServer { store };

    info!("Server listening on {}", addr);
    Server::builder()
        .add_service(BlindAuthServer::new(blind_auth))
        .serve(addr)
        .await?;

    Ok(())
}
