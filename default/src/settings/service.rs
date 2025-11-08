use std::{
    sync::{Arc, LazyLock, RwLock},
    time::Duration,
};

use tera::{Context, Tera};
use tera_hot_reload::watch;
use tower_http::services::ServeDir;
use tower_livereload::Reloader;
use uncovr::{prelude::ApiRouter, routing::get_service};

pub async fn statics() -> ApiRouter {
    let static_files = ServeDir::new("./public");
    ApiRouter::new().nest_service("/public", get_service(static_files))
}

#[cfg(debug_assertions)]
pub static LIVE_RELOADER: LazyLock<Reloader> = LazyLock::new(|| Reloader::new());

pub static TEMPLATE: LazyLock<Arc<RwLock<Tera>>> = LazyLock::new(|| {
    let tera = Tera::new("templates/**/*").expect("Failed to create Tera instance");
    let rw = Arc::new(RwLock::new(tera));

    #[cfg(debug_assertions)]
    {
        let tera_ref = Arc::clone(&rw);

        let _debouncer = watch(
            move || {
                if let Ok(mut t) = tera_ref.write() {
                    if let Err(err) = t.full_reload() {
                        eprintln!("Failed to reload templates: {}", err);
                    }
                }

                LIVE_RELOADER.reload();
            },
            Duration::from_millis(50),
            vec!["./templates"],
        );

        std::mem::forget(_debouncer);
    }

    rw
});

pub fn render(name: &str, context: &Context) -> String {
    TEMPLATE
        .read()
        .map(|tera| {
            tera.render(name, context)
                .unwrap_or_else(|_| "<h1>Internal Server Error</h1>".to_string())
        })
        .unwrap_or_else(|_| "<h1>Internal Server Error</h1>".to_string())
}
