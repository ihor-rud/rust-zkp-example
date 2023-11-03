use std::sync::Arc;

use api::Context;
use domain::auth::ZkpVerifier;
use gateways::MemoryAuthStorage;
use num_bigint::BigUint;
use tonic::transport::Server;

pub mod api;
pub mod domain;
pub mod gateways;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:50051".parse()?;
    let context = Context {
        auth_storage: Arc::new(MemoryAuthStorage::default()),
        verifier: Arc::new(ZkpVerifier {
            p: BigUint::from(23u64),
            g: BigUint::from(4u64),
            h: BigUint::from(9u64),
        }),
    };

    let greeter = api::ZkpAuth { context };

    Server::builder()
        .add_service(api::AuthServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
