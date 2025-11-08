use tera::Context as TemplateContext;
use uncovr::{prelude::*, response::Html};

use crate::settings::service::render;

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
        let mut context = TemplateContext::new();
        context.insert("name", "World");

        let html = render("index.html", &context);

        Html(html)
    }
}
