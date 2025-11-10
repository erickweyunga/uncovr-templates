use uncovr::{prelude::*, response::Html};
use wenzetu::{context, render};

#[derive(Clone)]
pub struct Index;

impl Endpoint for Index {
    fn ep(&self) -> Route {
        Route::GET("/")
    }
}

#[async_trait]
impl API for Index {
    type Req = ();
    type Res = Html<String>;

    async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
        let html = render(
            "index.html",
            &context! {
                name: "World",
            },
        );

        Html(html)
    }
}
