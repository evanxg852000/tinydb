use std::sync::Arc;

use anyhow::Result;
use tinykv::{kv::{server::TinyKvService, storage::standalone::StandaloneStorage}, proto::{self, tinykv::tiny_kv_server::TinyKvServer}};
use tonic::transport::Server;



#[tokio::main]
async fn main() -> Result<()> {
    println!("Welcome to KV");

    env_logger::init();


    let storage = Arc::new(StandaloneStorage::in_memory());
    let service = TinyKvService::new(storage);
    let addr = "127.0.0.1:8080".parse()?;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(reflection_service)
        .add_service(TinyKvServer::new(service))
        .serve(addr)
        .await?;


    Ok(())
}

