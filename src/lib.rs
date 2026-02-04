mod config;
use config::PluginConfig;

mod root;
use root::OAuthRedirectRoot;

mod context;
mod jwt;

use proxy_wasm::traits::*;
use proxy_wasm::types::*;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(OAuthRedirectRoot { config: PluginConfig::new() }) });
}}
