use std::path::Path;

use log::debug;
use wasmtime::{component::*, Config, Engine, Store};
use wasmtime_wasi::preview2::command;

use super::{RulesEngineState, WasmProduct, Error};

/// This struct is responsible for hosting the rules engine
/// it consists of a wasmtime component, store, and linker.
/// The store and linker are generics and used to wrap a State struct
/// that is used to store the state of the wasmtime runtime. 
pub struct LocalWasmtimeHost {
    component: Component, 
    store: Store<RulesEngineState>,
    linker: Linker<RulesEngineState>, 
}

/// The implementation of the LocalWasmtimeHost struct
/// and provides a constructor and execute method.
/// The execute method is a wrapper around a call into the WASM component.
impl LocalWasmtimeHost {
    pub fn new(actor_component_path: &Path) -> anyhow::Result<Self> {
        let config = Self::create_wasmtime_config();
        let engine = Self::create_wasmtime_engine(&config)?;
        let component = Self::create_wasmtime_component(&engine, actor_component_path)?;
        let store = Store::new(
            &engine,
            RulesEngineState::new(),
        );
        let mut linker:Linker<RulesEngineState> = Linker::new(&engine);
        command::sync::add_to_linker(&mut linker)?;
        super::ServiceHost::add_to_linker(&mut linker, |state: &mut RulesEngineState| state)?;
        Ok(Self {
            component,
            store,
            linker,
        })
    }

    /// This method is a wrapper around a call into the WASM component.
    /// It takes a Product as an input and returns a Result<Product, Error>
    /// An EngineInternalError is returned if there is an error instantiating the rules engine
    /// Otherwise, the result of the rules engine is returned.
    pub fn execute(&mut self, product: WasmProduct) -> Result<WasmProduct, Error> {

        debug!("Instantiating the rules engine");
        let (bindings, _) = match super::ServiceHost::instantiate(&mut self.store, &self.component, &self.linker) {
            Ok(bindings) => bindings,
            Err(e) => return Err(Error::EngineInternalError(e.to_string())),
        };
        
        debug!("Executing rules engine");
        let result = match bindings.aksstoredemo_rules_engine().call_execute(&mut self.store, &product) {
            Ok(result) => result,
            Err(e) => return Err(Error::EngineInternalError(e.to_string())),
        };

        match result {
            Ok(result) => return Ok(result),
            Err(e) => return Err(e),
        };
    }

    fn create_wasmtime_config() -> Config {
        let mut config = Config::new();
        config.wasm_component_model(true);
        return config;
    }

    fn create_wasmtime_engine(config: &Config) -> anyhow::Result<Engine> {
        return Engine::new(config);
    }

    fn create_wasmtime_component(engine: &Engine, actor_component_path: &Path) -> anyhow::Result<Component> {
        return Component::from_file(&engine, actor_component_path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{WasmProduct, Error};
    use std::path::PathBuf;
    use std::env;

    fn setup() -> LocalWasmtimeHost {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("./tests/rule_engine.wasm");
        LocalWasmtimeHost::new(&path).unwrap()
    }

    #[test]
    fn test_execute() {
        let mut host = setup();
        let product = WasmProduct {
            id: 123,
            name: "Sony WH-1000XM4".to_string(),
            brand: "Sony".to_string(),
            price: 348.0,
            description: "Industry-leading noise canceling over-ear headphones".to_string(),
            category: "Audio".to_string(),
            sku: "SONY-WH1000XM4".to_string(),
            quantity_available: 50,
            image_url: "/images/sony-wh1000xm4.jpg".to_string(),
            rating: 4.8,
            specifications: serde_json::json!({
                "type": "Over-ear",
                "battery_life": "30 hours",
                "noise_canceling": true,
                "wireless": true
            }).to_string(),
        };
        let result = host.execute(product).unwrap();
        assert_eq!(result.id, 123);
        assert_eq!(result.name, "Sony WH-1000XM4");
        assert_eq!(result.brand, "Sony");
        assert_eq!(result.price, 348.0);
        assert_eq!(result.category, "Audio");
    }

    #[test]
    fn test_execute_invalid_sku_format() {
        let mut host = setup();
        let product = WasmProduct {
            id: 123,
            name: "Test Product".to_string(),
            brand: "Test Brand".to_string(),
            price: 15.0,
            description: "This is a valid description".to_string(),
            category: "Electronics".to_string(),
            sku: "invalid".to_string(), // Invalid SKU format
            quantity_available: 10,
            image_url: "/placeholder.png".to_string(),
            rating: 0.0,
            specifications: "{}".to_string(),
        };
        let result = host.execute(product);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidProduct(error_message) => {
                assert!(error_message.contains("SKU format is invalid"));
            }
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_execute_negative_quantity() {
        let mut host = setup();
        let product = WasmProduct {
            id: 123,
            name: "Test Product".to_string(),
            brand: "Test Brand".to_string(),
            price: 15.0,
            description: "This is a valid description".to_string(),
            category: "Electronics".to_string(),
            sku: "TEST-SKU-123".to_string(),
            quantity_available: -5, // Negative quantity
            image_url: "/placeholder.png".to_string(),
            rating: 0.0,
            specifications: "{}".to_string(),
        };
        let result = host.execute(product);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidProduct(error_message) => {
                assert!(error_message.contains("Quantity cannot be negative"));
            }
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_execute_price_out_of_range() {
        let mut host = setup();
        let product = WasmProduct {
            id: 123,
            name: "Test Product".to_string(),
            brand: "Test Brand".to_string(),
            price: 10000.0, // Too high
            description: "This is a valid description".to_string(),
            category: "Electronics".to_string(),
            sku: "TEST-SKU-123".to_string(),
            quantity_available: 10,
            image_url: "/placeholder.png".to_string(),
            rating: 0.0,
            specifications: "{}".to_string(),
        };
        let result = host.execute(product);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::PricingStandardsViolation(error_message) => {
                assert!(error_message.contains("Price is outside allowed range"));
            }
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_execute_empty_brand() {
        let mut host = setup();
        let product = WasmProduct {
            id: 123,
            name: "Test Product".to_string(),
            brand: "".to_string(), // Empty brand
            price: 15.0,
            description: "This is a valid description".to_string(),
            category: "Electronics".to_string(),
            sku: "TEST-SKU-123".to_string(),
            quantity_available: 10,
            image_url: "/placeholder.png".to_string(),
            rating: 0.0,
            specifications: "{}".to_string(),
        };
        let result = host.execute(product);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidProduct(error_message) => {
                assert!(error_message.contains("Brand is required"));
            }
            _ => panic!("Unexpected error type"),
        }
    }
}