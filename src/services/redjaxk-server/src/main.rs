use axum::{Router, routing::get};
use maud::{DOCTYPE, Markup, html};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/fragments/nodes", get(node_status));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "Redjaxk" }
                script src="/static/htmx.min.js" {}
            }
            body {
                h1 { "Dashboard" }
                (node_status().await)
            }
        }
    }
}

async fn node_status() -> Markup {
    html! {
        div
            hx-get="/fragments/nodes"
            hx-trigger="every 5s"
            hx-swap="outerHTML"
        {
            p { "node-a: Online" }
            p { "node-b: Online" }
        }
    }
}
