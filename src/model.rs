use serde::{Deserialize, Serialize};
use crate::localwasmtime::WasmProduct;

#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub brand: String,
    pub price: f64,
    pub description: String,
    pub category: String,
    pub sku: String,
    pub in_stock: bool,
    pub quantity_available: i32,
    pub image_url: String,
    pub rating: f32,
    pub specifications: serde_json::Value,
}

#[derive(Deserialize)]
pub struct ProductInfo {
    pub product_id: i32,
}

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub brand: String,
    pub price: f64,
    pub description: String,
    pub category: String,
    pub sku: String,
    pub quantity_available: i32,
    pub image_url: String,
    pub specifications: serde_json::Value,
}

#[derive(Serialize)]
pub struct InventoryResponse {
    pub product_id: i32,
    pub in_stock: bool,
    pub quantity_available: i32,
}

// Fix the Into implementation
impl From<Product> for WasmProduct {
    fn from(product: Product) -> Self {
        WasmProduct {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price as f32,
            image: product.image_url,
        }
    }
}

// Fix the From implementation
impl From<WasmProduct> for Product {
    fn from(product: WasmProduct) -> Self {
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