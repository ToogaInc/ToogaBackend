use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddUserBody {
    #[serde(rename = "discordId")]
    pub discord_id: String,
    #[serde(rename = "ign")]
    pub ign: String,
}
