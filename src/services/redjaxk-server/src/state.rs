use redjaxk_protocol::v1::node_agent_client::NodeAgentClient;
use tonic::transport::Channel;

#[derive(Clone)]
pub struct AppState {
    pub agent: NodeAgentClient<Channel>,
}
