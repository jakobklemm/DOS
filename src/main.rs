mod error;
use error::TABError;

mod routes;
use routes::Application;

mod handlers;
mod models;
mod states;

mod translator;

use tracing::info;

#[tokio::main]
async fn main() -> Result<(), TABError> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("TODO: panic message");

    let app = Application::new();

    let _ = translator::Dictionary::load();

    info!("Starting web server");

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
