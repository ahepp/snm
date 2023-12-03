use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(app_endpoint));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn app_endpoint() -> Html<String> {
    Html(dioxus_ssr::render_lazy(rsx! {
        div { "Hello, World!" }
    }))
}
