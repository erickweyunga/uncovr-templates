use crate::{AppState, api::health::Health};
use uncovr::{api::ApiResponse, config::AppConfig, prelude::ApiRouter, server::Server};

pub async fn create_api_routes(_state: AppState, api_config: AppConfig) -> ApiRouter {
    let api = Server::new()
        .with_config(api_config)
        .register(Health::new("OK".to_string()));

    api.fallback(handle_api_fallback).build().into_router()
}

async fn handle_api_fallback() -> ApiResponse<()> {
    ApiResponse::NotFound {
        code: "not_found",
        message: "Not Found".to_string(),
    }
}
