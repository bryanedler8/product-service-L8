use crate::model::ProductInfo;
use crate::startup::AppState;
use actix_web::{web, Error, HttpResponse};
use serde_json::json;

pub async fn delete_product(
    data: web::Data<AppState>,
    path: web::Path<ProductInfo>,
) -> Result<HttpResponse, Error> {
    let mut products = data.products.lock().unwrap();

    // Find product by id in products
    let index = products
        .iter()
        .position(|p| p.id == path.product_id);

    match index {
        Some(i) => {
            let removed_product = products.remove(i);
            Ok(HttpResponse::Ok().json(json!({
                "message": "Product deleted successfully",
                "deleted_product": removed_product
            })))
        }
        None => {
            Ok(HttpResponse::NotFound().json(json!({
                "error": "Product not found",
                "product_id": path.product_id
            })))
        }
    }
}