use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Represents an association between a Discord ID and an IGN.
#[derive(Serialize, Deserialize)]
pub struct User {
    /// The Discord ID.
    #[serde(rename = "discordId")]
    discord_id: String,

    /// The in-game name.
    ign: String,

    /// The in-game name, but all lowercase for easy querying.
    ///
    /// This value is expected to be unique.
    #[serde(rename = "ignLower")]
    ign_lower: String,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[{}] {}", self.discord_id, self.ign)
    }
}
