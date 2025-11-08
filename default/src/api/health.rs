use derive_new::new;
use uncovr::prelude::*;

#[derive(Clone, new)]
pub struct Health {
    pub status: String,
}

impl Endpoint for Health {
    fn ep(&self) -> Route {
        Route::GET("/health")
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("Check Application Health")
                .responses(|op| op.response::<200, Json<String>>()),
        )
    }
}

#[async_trait]
impl API for Health {
    type Req = ();
    type Res = ApiResponse<String>;

    async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
        let health = Health::new("OK".to_string());
        ApiResponse::Ok(health.status)
    }
}
