use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct UpdateInfo {
    pub sha: String,
}
