use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct PluginConfig {
    pub header_content: String,
}