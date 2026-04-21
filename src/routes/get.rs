use crate::model::ProductInfo;
use crate::startup::AppState;
use actix_web::{web, Error, HttpResponse};
use serde_json::json;

pub async fn get_product(
    data: web::Data<AppState>,
    path: web::Path<ProductInfo>,
) -> Result<HttpResponse, Error> {
    let products = data.products.lock().unwrap();

    // find product by id in products
    let index = products
        .iter()
        .position(|p| p.id == path.product_id);
    if let Some(i) = index {
        return Ok(HttpResponse::Ok().json(products[i].clone()))
    }
    else {
        return Ok(HttpResponse::NotFound().json(json!({
            "error": "Product not found",
            "product_id": path.product_id
        })))
    }
}

pub async fn get_products(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let products = data.products.lock().unwrap();
    
    // Optional: Add filtering by category or in_stock status
    // For now, return all products
    Ok(HttpResponse::Ok().json(products.to_vec()))
}

// New endpoint: Get products by category
pub async fn get_products_by_category(
    data: web::Data<AppState>,
    category: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let products = data.products.lock().unwrap();
    let filtered: Vec<_> = products
        .iter()
        .filter(|p| p.category.eq_ignore_ascii_case(&category))
        .cloned()
        .collect();
    
    Ok(HttpResponse::Ok().json(filtered))
}

// New endpoint: Check inventory for a product
pub async fn check_inventory(
    data: web::Data<AppState>,
    path: web::Path<ProductInfo>,
) -> Result<HttpResponse, Error> {
    let products = data.products.lock().unwrap();
    
    let product = products.iter().find(|p| p.id == path.product_id);
    match product {
        Some(p) => {
            let inventory_status = json!({
                "product_id": p.id,
                "name": p.name,
                "in_stock": p.in_stock,
                "quantity_available": p.quantity_available,
                "sku": p.sku
            });
            Ok(HttpResponse::Ok().json(inventory_status))
        }
        None => Ok(HttpResponse::NotFound().json(json!({
            "error": "Product not found"
        })))
    }
}