use std::net::SocketAddr;
use std::process::ExitCode;
use std::str::FromStr;

use crate::api::create_router;
use crate::config::EnvConfig;
use crate::state::{AppState, InternalState};
use dotenv::dotenv;

mod api;
mod config;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    dotenv().ok();
    let config = EnvConfig::init();

    let addr = SocketAddr::from_str(
        format!("{}:{}", config.api_address.as_str(), config.api_port).as_str(),
    );

    let state = AppState::new(InternalState::new(config));

    axum::Server::bind(&addr.unwrap())
        .serve(create_router(state).into_make_service())
        .await
        .unwrap();
    Ok(ExitCode::SUCCESS)
}
