use std::sync::Arc;

use mongodb::{options::ClientOptions, Client};

use crate::config::EnvConfig;

pub type AppState = Arc<InternalState>;

pub struct InternalState {
    /// The configuration data from the `.env` file.
    pub env: EnvConfig,

    /// The MongoDB client used to interact with the database.
    pub mongo_client: Client,
}

impl InternalState {
    /// Creates a new `InternalState` with the specified configuration object.
    ///
    /// # Parameters
    /// - `config`: The configuration object.
    ///
    /// # Returns
    /// The `InternalState`.
    pub async fn init(config: EnvConfig) -> anyhow::Result<Self> {
        let options = ClientOptions::parse(config.mongo_uri.as_str()).await?;
        Ok(InternalState {
            env: config,
            mongo_client: Client::with_options(options)?,
        })
    }
}
