use uncovr::{prelude::ApiRouter, server::Server};

use crate::{AppState, api::health::Health, app::index::Index};

pub async fn create_routes(_state: AppState) -> ApiRouter {
    let app = Server::new().register(Index);

    app.build().into_router()
}

pub async fn create_api_routes(_state: AppState) -> ApiRouter {
    let api = Server::new().register(Health::new("OK".to_string()));

    api.build().into_router()
}
