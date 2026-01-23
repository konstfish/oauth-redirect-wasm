use crate::config::PluginConfig;

use proxy_wasm::traits::*;
use proxy_wasm::types::*;

use log::info;

pub struct OAuthRedirect {
    pub context_id: u32,
    pub config: PluginConfig,
}

impl Context for OAuthRedirect {}

impl HttpContext for OAuthRedirect {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        /*for (name, value) in &self.get_http_request_headers() {
            info!("#{} -> {}: {}", self.context_id, name, value);
        }*/

        match self.get_http_request_header(":path") {
            Some(path) if path == "/hello" => {
                self.send_http_response(
                    200,
                    vec![
                        ("Hello", &self.config.header_content),
                        ("Powered-By", "proxy-wasm"),
                    ],
                    Some(&self.config.header_content.as_bytes()),
                );
                Action::Pause
            }
            _ => Action::Continue,
        }
    }

    /*fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
        /*for (name, value) in &self.get_http_response_headers() {
            info!("#{} <- {}: {}", self.context_id, name, value);
        }*/
        Action::Continue
    }*/

    fn on_log(&mut self) {
        info!("#{} completed.", self.context_id);
    }
}
