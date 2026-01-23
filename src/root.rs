use crate::config::PluginConfig;
use crate::context::OAuthRedirect;

use proxy_wasm::traits::*;
use proxy_wasm::types::*;

use log::info;
pub struct OAuthRedirectRoot {
    pub config: PluginConfig,
}

impl Context for OAuthRedirectRoot {}

impl RootContext for OAuthRedirectRoot {
    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        info!("OAuthRedirectRoot::on_configure called");

        if let Some(config_bytes) = self.get_plugin_configuration() {
            if let Ok(config) = serde_json::from_slice::<PluginConfig>(&config_bytes) {
                self.config = config;
            } else {
                info!("Failed to parse configuration, using defaults");
            }
        }
        true
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(OAuthRedirect {
            context_id,
            config: self.config.clone(),
        }))
    }
}
