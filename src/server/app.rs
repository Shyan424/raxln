use std::sync::{OnceLock, RwLock};

use config::Config;
use tracing::Level;

use super::router::router;

pub struct App;

impl App {
    pub fn new () -> Self {
        set_tracing();
        config();

        App{}
    }

    pub async fn start(&self) {
        let conf_port = config().read().unwrap().get::<i32>("app.port");
        let port = if let Ok(p) = conf_port {
            p
        } else {
            3000
        };

        axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
            .serve(router().into_make_service())
            .await
            .unwrap();
    }
}

fn set_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
}

fn config() -> &'static RwLock<Config> {
    static CONF: OnceLock<RwLock<Config>> = OnceLock::new();

    CONF.get_or_init(||
        RwLock::new(
            Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .build()
            .expect("Read config Error")
        )
    )
}

#[cfg(test)]
mod test {
    use config::Config;


    #[test]
    fn config_test() {
        let p = Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .build().expect("no conf");

        println!("server port {}", p.get_string("app.port").expect("app.port error"));
    }

}