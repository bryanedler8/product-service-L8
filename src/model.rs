use serde::{Deserialize, Serialize};
use crate::localwasmtime::WasmProduct;

#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub brand: String,
    pub price: f64,  // Changed to f64 for more precision with electronics pricing
    pub description: String,
    pub category: String,  // e.g., "Laptops", "Smartphones", "TVs", "Audio", "Gaming"
    pub sku: String,  // Stock Keeping Unit - unique identifier
    pub in_stock: bool,
    pub quantity_available: i32,
    pub image_url: String,
    pub rating: f32,  // Average customer rating (0-5)
    pub specifications: serde_json::Value,  // Flexible JSON for tech specs (RAM, storage, etc.)
}

#[derive(Deserialize)]
pub struct ProductInfo {
    pub product_id: i32,
}



impl Into<WasmProduct> for Product {
    fn into(self) -> WasmProduct {
        WasmProduct {
            id: self.id,
            name: self.name,
            description: self.description,
            price: self.price as f32,  // Cast to f32 for WasmProduct compatibility
            image: self.image_url,
        }
    }
}

impl From<WasmProduct> for Product {
    fn from(product: WasmProduct) -> Self {
        // Provide default values for Best Buy-specific fields when converting from WasmProduct
        Self {
            id: product.id,
            name: product.name,
            brand: String::from("Generic"),
            price: product.price as f64,
            description: product.description,
            category: String::from("Electronics"),
            sku: format!("SKU-{}", product.id),
            in_stock: true,
            quantity_available: 100,
            image_url: product.image,
            rating: 0.0,
            specifications: serde_json::json!({}),
        }
    }
}