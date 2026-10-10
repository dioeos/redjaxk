pub mod v1 {
    tonic::include_proto!("redjaxk.v1");
}

pub mod podgate {
    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    pub enum Request {
        ListContainers,
    }

    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    pub enum Response {
        Containers { containers: Vec<PodmanContainer> },
        Error { request: Request, message: String },
    }

    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    pub struct PodmanContainer {
        pub id: String,
        pub image: String,
        pub redjaxk_name: String,
        pub redjaxk_url: String,
    }
}
