use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct VersionResponse {
    pub version: String,
}