use std::env;

pub struct EnvConfig {
    pub api_address: String,
    pub api_port: usize,
}

impl EnvConfig {
    /// Initializes the `EnvConfig` object with the configuration information from the `.env`
    /// file.
    ///
    /// # Returns
    /// The `EnvConfig` object.
    pub fn init() -> Self {
        EnvConfig {
            api_address: env::var("API_ADDRESS")
                .expect("You need an environmental variable for `API_ADDRESS`"),
            api_port: env::var("API_PORT")
                .expect("You need an environmental variable for `API_PORT`")
                .parse()
                .expect("The port specified for `API_PORT` is not a valid unsigned integer."),
        }
    }
}
