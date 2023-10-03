use std::env;

pub struct EnvConfig {
    pub api_address: String,
    pub api_port: usize,
    pub mongo_uri: String,
    pub db_name: String,
    pub collections: CollectionIds,
}

pub struct CollectionIds {
    pub user_id_name: String,
    pub guild_role: String,
    pub guild_channel: String,
    pub quota_config: String,
    pub quota_rules: String,
    pub quota_logged: String,
    pub member_punishments: String,
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
            mongo_uri: env::var("MONGO_CONN_URI")
                .expect("You need an environmental variable for `MONGO_CONN_URI`"),
            db_name: env::var("DATABASE_NAME")
                .expect("You need an environmental variable for `DATABASE_NAME`"),
            collections: CollectionIds {
                user_id_name: env::var("USER_ID_NAME_COLL")
                    .expect("You need an environmental variable for 'USER_ID_NAME_COLL'"),
                guild_role: env::var("GUILD_ROLE_COLL")
                    .expect("You need an environmental variable for 'GUILD_ROLE_COLL'"),
                guild_channel: env::var("GUILD_CHANNEL_COLL")
                    .expect("You need an environmental variable for 'GUILD_CHANNEL_COLL'"),
                quota_config: env::var("QUOTA_CONFIG_COLL")
                    .expect("You need an environmental variable for 'QUOTA_CONFIG_COLL'"),
                quota_rules: env::var("QUOTA_RULES_COLL")
                    .expect("You need an environmental variable for 'QUOTA_RULES_COLL'"),
                quota_logged: env::var("QUOTA_LOGGED_COLL")
                    .expect("You need an environmental variable for 'QUOTA_LOGGED_COLL'"),
                member_punishments: env::var("MEMBER_PUNISHMENTS_COLL")
                    .expect("You need an environmental variable for 'MEMBER_PUNISHMENTS_COLL'"),
            },
        }
    }
}
