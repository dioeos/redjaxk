use redjaxk_protocol::v1::node_agent_server::NodeAgentServer;
use tonic::transport::Server;

use crate::service::AgentService;

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node_id = std::env::var("REDJAXK_NODE_ID")?;

    let addr = std::env::var("REDJAXK_BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:50051".into())
        .parse()?;

    let agent = AgentService { node_id };

    Server::builder()
        .add_service(NodeAgentServer::new(agent))
        .serve(addr)
        .await?;

    Ok(())
}
