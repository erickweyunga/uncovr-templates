use wenzetu::prelude::*;

mod api;

#[derive(Clone)]
pub struct AppState;

#[tokio::main]
async fn main() {
    let api_config = helpers::api_config(
        "API",
        "0.1.0",
        "127.0.0.1:3000",
        Environment::Development,
        "/api-docs",
        "/openapi.json",
    );

    let api_routes = api::routes::create_api_routes(AppState, api_config).await;

    App::new()
        .no_static_files()
        .docs_path("/docs")
        .api("/api", api_routes)
        .live_reload(true)
        .serve()
        .await
        .unwrap();
}
