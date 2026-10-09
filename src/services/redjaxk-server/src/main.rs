mod state;

use std::time::Duration;

use axum::{Router, extract::State, routing::get};
use maud::{DOCTYPE, Markup, html};
use redjaxk_protocol::v1::{StatusRequest, node_agent_client::NodeAgentClient};
use tonic::transport::Channel;
use tower_http::services::ServeDir;

use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    let channel = Channel::from_static("http://ishtar1:50051")
        .connect()
        .await?;

    let state = AppState {
        agent: NodeAgentClient::new(channel),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/fragments/nodes", get(node_status))
        .nest_service(
            "/static", 
            ServeDir::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/static"
            )))
        .with_state(state);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn check_agent_online(state: &AppState) -> bool {
    let mut client = state.agent.clone();

    tokio::time::timeout(Duration::from_secs(2), client.get_status(StatusRequest {}))
        .await
        .is_ok_and(|res| res.is_ok())
}

fn render_node_status(online: bool) -> Markup {
    html! {
        p {
            "ishtar1: "
            (if online { "Online" } else { "Offline" })
        }
    }
}

async fn index(State(state): State<AppState>) -> Markup {
    let online = check_agent_online(&state).await;

    html! {
        (DOCTYPE)
        html {
            head {
                title { "Redjaxk" }
                script src="/static/htmx.min.js" {}
            }
            body {
                h1 { "Dashboard" }
                
                button
                    hx-get="/fragments/nodes"
                    hx-target="#node-status"
                    hx-swap="innerHTML"
                {
                    "Refresh"
                }

                div id="node-status" {
                    (render_node_status(online))
                }
            }
        }
    }
}

async fn node_status(State(state): State<AppState>) -> Markup {
    let online = check_agent_online(&state).await;
    render_node_status(online)
}
