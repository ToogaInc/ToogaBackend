use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Represents an association between a Discord ID and an IGN.
#[derive(Serialize, Deserialize)]
pub struct User {
    /// The Discord ID.
    #[serde(rename = "discordId")]
    pub discord_id: String,

    /// The in-game name.
    pub ign: String,

    /// The in-game name, but all lowercase for easy querying.
    ///
    /// This value is expected to be unique.
    #[serde(rename = "ignLower")]
    pub ign_lower: String,
}

impl User {
    /// Creates a new instance of this user with the specified Discord ID and
    /// in-game name.
    ///
    /// # Parameters
    /// - `discord_id`: The Discord ID.
    /// - `ign`: The in-game name, exactly as formatted in-game.
    ///
    /// # Returns
    /// The user object.
    pub fn new<S>(discord_id: S, ign: S) -> Self
    where
        S: Into<String>,
    {
        let ign = ign.into();
        User {
            discord_id: discord_id.into(),
            ign_lower: ign.to_lowercase(),
            ign,
        }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[{}] {}", self.discord_id, self.ign)
    }
}
