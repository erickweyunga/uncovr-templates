use crate::settings::vars::AppConfig;
use config::{Config, Environment};
use lazy_static::lazy_static;

pub mod service;
pub mod vars;

lazy_static! {
    pub static ref CONFIG: AppConfig = {
        let _ = dotenvy::dotenv();

        let source = Environment::default().separator(".");

        let config = Config::builder().add_source(source).build();

        if let Err(e) = config {
            panic!("{e}");
        }

        let config = config.unwrap().try_deserialize::<AppConfig>();

        if let Err(e) = config {
            panic!("{e}");
        }

        config.unwrap()
    };
}
