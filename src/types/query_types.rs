use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IgnQuery {
    #[serde(rename = "ign")]
    pub ign: String,
}
