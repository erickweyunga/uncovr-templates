use tera::Context;
use uncovr::{config::AppConfig, prelude::ApiRouter, response::Html, server::Server};
use wenzetu::render;

use crate::{AppState, app::index::Index};

pub async fn create_routes(_state: AppState, app_config: AppConfig) -> ApiRouter {
    let app = Server::new().with_config(app_config).register(Index);

    app.fallback(handle_app_fallback).build().into_router()
}

async fn handle_app_fallback() -> Html<String> {
    let html = render("not_found.html", &Context::new());
    Html(html)
}
