mod error;
mod podgate;
mod service;

use redjaxk_protocol::v1::node_agent_server::NodeAgentServer;
use tonic::transport::Server;
use tracing_subscriber::{EnvFilter, fmt};

use crate::{podgate::PodgateClient, service::AgentService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let format = fmt::format().with_level(true).with_target(true).compact();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .event_format(format)
        .init();

    let node_id = std::env::var("REDJAXK_NODE_ID")?;

    let addr = std::env::var("REDJAXK_BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:50051".into())
        .parse()?;

    let podgate_client = PodgateClient::connect().await?;

    let agent = AgentService {
        node_id,
        podgate_client,
    };

    Server::builder()
        .add_service(NodeAgentServer::new(agent))
        .serve(addr)
        .await?;

    Ok(())
}
