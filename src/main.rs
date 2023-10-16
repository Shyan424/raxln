
use learn_axum::server::app::App;

#[tokio::main]
async fn main() {
    App::new().start().await;
}
