use wenzetu::prelude::*;

mod api;
mod app;

#[derive(Clone)]
pub struct AppState;

#[tokio::main]
async fn main() {
    let config = load_config();
    let (web_config, api_config) = helpers::fullstack_configs(&config);

    let web_routes = app::routes::create_routes(AppState, web_config).await;
    let api_routes = api::routes::create_api_routes(AppState, api_config).await;

    App::new()
        .auto_config()
        .web(web_routes)
        .api("/api", api_routes)
        .serve()
        .await
        .unwrap();
}
