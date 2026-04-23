use serde::Deserialize;
use std::net::TcpListener;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub wasm_rules_engine_enabled: bool,
    pub wasm_bin_path: PathBuf,
    pub max_size: usize,
    pub ai_service_url: String,
    pub bestbuy_categories: Vec<String>,
    pub max_product_price: f64,
    pub min_product_price: f64,
    pub sku_format_regex: String,
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn set_wasm_rules_engine(mut self, enabled: bool) -> Self {
        self.wasm_rules_engine_enabled = enabled;
        self
    }
    
    pub fn get_tcp_listener(&self) -> std::io::Result<TcpListener> {
        let addr = format!("{}:{}", self.host, self.port);
        TcpListener::bind(&addr)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3002,
            log_level: "info".to_string(),
            wasm_rules_engine_enabled: false,
            wasm_bin_path: PathBuf::from("./tests/rule_engine.wasm"),
            max_size: 1024 * 1024, // 1MB
            ai_service_url: "http://localhost:3001".to_string(),
            bestbuy_categories: vec![
                "Laptops".to_string(),
                "Smartphones".to_string(),
                "TVs".to_string(),
                "Audio".to_string(),
                "Gaming".to_string(),
                "Tablets".to_string(),
                "Wearables".to_string(),
            ],
            max_product_price: 10000.0,
            min_product_price: 0.01,
            sku_format_regex: r"^[A-Z]{2,4}-[A-Z0-9]{3,10}-[A-Z0-9]{3,10}$".to_string(),
        }
    }
}