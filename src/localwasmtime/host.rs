use std::path::Path;
use log::debug;
use super::{WasmProduct, Error};  // Remove RulesEngineState from import

/// Simplified host for the rules engine (WASM disabled)
pub struct LocalWasmtimeHost {
    enabled: bool,
}

impl LocalWasmtimeHost {
    pub fn new(_actor_component_path: &Path) -> anyhow::Result<Self> {
        debug!("Creating LocalWasmtimeHost with WASM disabled");
        Ok(Self {
            enabled: false,
        })
    }

    pub fn execute(&mut self, product: WasmProduct) -> Result<WasmProduct, Error> {
        debug!("WASM rules engine disabled, returning product unchanged");
        Ok(product)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        let mut host = LocalWasmtimeHost::new(Path::new("./tests/rule_engine.wasm")).unwrap();
        let product = WasmProduct {
            id: 123,
            name: "Test Product".to_string(),
            description: "This is a test".to_string(),
            price: 15.0,
            image: "/placeholder.png".to_string(),
        };
        let result = host.execute(product).unwrap();
        assert_eq!(result.id, 123);
        assert_eq!(result.name, "Test Product");
    }
}