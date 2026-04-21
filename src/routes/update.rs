use actix_web::{error, web, Error, HttpResponse};
use crate::model::Product;
use crate::startup::AppState;
use futures_util::StreamExt;
use crate::localwasmtime::validate_product;

pub async fn update_product(
    data: web::Data<AppState>,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    let mut products = data.products.lock().unwrap();

    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > data.settings.max_size {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let mut product = serde_json::from_slice::<Product>(&body)?;
    
    // Ensure in_stock is consistent with quantity_available
    product.in_stock = product.quantity_available > 0;
    
    match validate_product(&data.settings, &product) {
        Ok(validated_product) => {
            let index = products.iter().position(|p| p.id == product.id)
                .ok_or_else(|| error::ErrorNotFound("Product not found"))?;
            products[index] = validated_product.clone();
            Ok(HttpResponse::Ok().json(validated_product))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().body(e.to_string()))
        }
    }  
}