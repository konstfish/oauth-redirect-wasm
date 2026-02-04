use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct PluginConfig {
    pub redirect_url: String,
    pub redirect_param: String,
    pub add_redirect_param: Option<bool>,
    pub expiry_margin_minutes: Option<i64>,
    #[serde(skip)]
    pub redirect_prefix: String,
}

impl PluginConfig {
    pub fn new() -> Self {
        PluginConfig {
            redirect_url: String::new(),
            redirect_param: String::new(),
            add_redirect_param: Some(true),
            expiry_margin_minutes: Some(2),
            redirect_prefix: String::new(),
        }
    }

    pub fn init(&mut self) {
        self.redirect_prefix = format!("{}?{}=", self.redirect_url, self.redirect_param);
    }
}