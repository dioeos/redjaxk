use redjaxk_protocol::v1::{StatusReply, StatusRequest, node_agent_server::NodeAgent};
use tonic::{Request, Response, Status};

pub struct AgentService {
    pub node_id: String,
}

#[tonic::async_trait]
impl NodeAgent for AgentService {
    async fn get_status(
        &self,
        _request: Request<StatusRequest>,
    ) -> Result<Response<StatusReply>, Status> {
        Ok(Response::new(StatusReply {
            node_id: self.node_id.clone(),
        }))
    }
}
