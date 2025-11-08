use crate::{
    routes::{create_api_routes, create_routes},
    settings::{CONFIG, service::statics},
};

use derive_new::new;
use uncovr::{config::AppConfig, server::Server};

mod api;
mod app;
mod routes;
mod settings;

#[derive(Clone, new)]
pub struct AppState;

#[tokio::main]
async fn main() {
    let config = &*CONFIG;
    let addr = format!("{}:{}", config.app.address, config.app.port);

    let app_config = AppConfig::new(config.app.name.clone(), config.app.version.clone())
        .bind(addr)
        .description(config.app.description.clone())
        .environment(config.environment.clone());

    let state = AppState::new();

    let app_routes = create_routes(state.clone()).await;
    let api_routes = create_api_routes(state.clone()).await;
    let static_routes = statics().await;

    let mut server = Server::new()
        .with_config(app_config)
        .merge(app_routes)
        .merge(api_routes)
        .merge(static_routes);

    #[cfg(debug_assertions)]
    {
        use tera_hot_reload::LiveReloadLayer;
        server = server.layer(LiveReloadLayer::new());
    }

    server
        .build()
        .serve()
        .await
        .expect("Failed to start server");
}
