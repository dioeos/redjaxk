use redjaxk_protocol::v1::{
    Container, ListContainersReply, ListContainersRequest, StatusReply, StatusRequest, node_agent_server::NodeAgent
};
use tonic::Status;
use tracing::error;

use crate::podgate::PodgateClient;

pub struct AgentService {
    pub node_id: String,
    pub podgate_client: PodgateClient,
}

#[tonic::async_trait]
impl NodeAgent for AgentService {
    async fn get_status(
        &self,
        _request: tonic::Request<StatusRequest>,
    ) -> Result<tonic::Response<StatusReply>, Status> {
        Ok(tonic::Response::new(StatusReply {
            node_id: self.node_id.clone(),
        }))
    }

    async fn list_containers(
        &self,
        _request: tonic::Request<ListContainersRequest>,
    ) -> Result<tonic::Response<ListContainersReply>, Status> {
        let containers = self
            .podgate_client
            .list_containers()
            .await
            .map_err(|err| {
                error!(?err, "Podman helper failed");
                Status::unavailable("Podman discovery unavailable")
            })?;

        //convert local IPC models to gRPC variant
        let containers = containers
            .into_iter()
            .map(|container| Container {
                id: container.id,
                image: container.image,
                redjaxk_name: container.redjaxk_name,
                redjaxk_url: container.redjaxk_url,
            })
            .collect();

        Ok(tonic::Response::new(ListContainersReply {
            containers,
        }))
    }
}
