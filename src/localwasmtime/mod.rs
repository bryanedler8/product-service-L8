mod rules_engine_state;
mod host;

use std::fmt;
use std::path::Path;
use log::info;
use crate::model::Product;
use crate::configuration::Settings;

pub use rules_engine_state::RulesEngineState;
pub use host::LocalWasmtimeHost;

#[derive(Debug, Clone)]
pub struct WasmProduct {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f32,
    pub image: String,
}

#[derive(Debug)]
pub enum Error {
    InvalidProduct(String),
    PricingStandardsViolation(String),
    EngineInternalError(String),
}

// Implement Display for Error
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidProduct(msg) => write!(f, "Invalid Product: {}", msg),
            Error::PricingStandardsViolation(msg) => write!(f, "Pricing Standards Violation: {}", msg),
            Error::EngineInternalError(msg) => write!(f, "Engine Internal Error: {}", msg),
        }
    }
}

// Implement std::error::Error for Error
impl std::error::Error for Error {}

fn wasm_bin_path_exists(wasm_bin_path: &Path) -> bool {
    let wasm_bin_path_exists = wasm_bin_path.exists();
    info!("WASM rules engine path exists: {}", wasm_bin_path_exists);
    return wasm_bin_path_exists;
}

pub fn validate_product(_settings: &Settings, product: &Product) -> Result<Product, Error>{
    info!("WASM rules engine disabled, skipping validation");
    Ok(product.clone())
}