use std::sync::Arc;

use crate::config::EnvConfig;

pub type AppState = Arc<InternalState>;

pub struct InternalState {
    /// The configuration data from the `.env` file.
    pub env: EnvConfig,
}

impl InternalState {
    /// Creates a new `InternalState` with the specified configuration object.
    ///
    /// # Parameters
    /// - `config`: The configuration object.
    ///
    /// # Returns
    /// The `InternalState`.
    pub fn new(config: EnvConfig) -> Self {
        InternalState { env: config }
    }
}
