use crate::config::PluginConfig;

use proxy_wasm::traits::*;
use proxy_wasm::types::*;

use log::{info, debug};

use crate::jwt;

pub struct OAuthRedirect {
    pub context_id: u32,
    pub config: PluginConfig,
}

impl Context for OAuthRedirect {}

impl HttpContext for OAuthRedirect {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        if !self.config.contains_host(
            &self.get_http_request_header(":authority").unwrap_or_default()
        ) {
            info!("#{} host is not in redirect_hosts, skipping redirect logic", self.context_id);
            return Action::Continue;
        }

        let auth_token = self.get_http_request_header("cookie")
            .and_then(|c| {
                c.split(';')
                    .map(|s| s.trim())
                    .find(|s| s.starts_with("Authorization="))
                    .and_then(|s| s.strip_prefix("Authorization="))
                    .map(|s| s.to_string())
            });

        if let Some(ref token) = auth_token {
            debug!("#{} has token: {}", self.context_id, token);
            if jwt::is_valid_within(token, self.config.expiry_margin_minutes.unwrap_or(2)) {
                debug!("#{} token valid within margin, redirecting", self.context_id);
                return Action::Continue;
            }
        }
                
        let redirect: String = if self.config.add_redirect_param.unwrap_or(true) {
            let original_url = format!(
                "{}://{}{}",
                self.get_http_request_header(":scheme").unwrap_or_default(),
                self.get_http_request_header(":authority").unwrap_or_default(),
                self.get_http_request_header(":path").unwrap_or_default(),
            );

            format!("{}{}", self.config.redirect_prefix, original_url)
        } else {
            format!("{}",self.config.redirect_url,)
        };

        debug!("#{} no valid authorization, redirecting to {}", self.context_id, redirect);

        self.send_http_response(
            302,
            vec![
                ("location", redirect.as_str()),
                ("cache-control", "no-cache"),
            ],
            None,
        );
        Action::Pause
    }

    /*fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
        /*for (name, value) in &self.get_http_response_headers() {
            info!("#{} <- {}: {}", self.context_id, name, value);
        }*/
        Action::Continue
    }*/

    /*fn on_log(&mut self) {
        info!("#{} completed.", self.context_id);
    }*/
}
