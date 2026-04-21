use crate::model::ProductInfo;
use crate::startup::AppState;
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct UpdateInventoryRequest {
    pub quantity_change: i32, // Can be positive (add stock) or negative (remove stock)
}

#[derive(Serialize)]
pub struct InventoryUpdateResponse {
    pub product_id: i32,
    pub previous_quantity: i32,
    pub new_quantity: i32,
    pub in_stock: bool,
    pub message: String,
}

pub async fn update_inventory(
    data: web::Data<AppState>,
    path: web::Path<ProductInfo>,
    req: web::Json<UpdateInventoryRequest>,
) -> Result<HttpResponse, Error> {
    let mut products = data.products.lock().unwrap();
    
    let product = products.iter_mut().find(|p| p.id == path.product_id);
    
    match product {
        Some(p) => {
            let previous_quantity = p.quantity_available;
            let new_quantity = previous_quantity + req.quantity_change;
            
            if new_quantity < 0 {
                return Ok(HttpResponse::BadRequest().json(json!({
                    "error": "Insufficient inventory",
                    "available": previous_quantity,
                    "requested": -req.quantity_change
                })));
            }
            
            p.quantity_available = new_quantity;
            p.in_stock = new_quantity > 0;
            
            Ok(HttpResponse::Ok().json(InventoryUpdateResponse {
                product_id: p.id,
                previous_quantity,
                new_quantity,
                in_stock: p.in_stock,
                message: format!("Inventory updated successfully. New quantity: {}", new_quantity),
            }))
        }
        None => Ok(HttpResponse::NotFound().json(json!({
            "error": "Product not found"
        })))
    }
}