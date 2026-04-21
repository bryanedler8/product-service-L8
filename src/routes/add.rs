use actix_web::{error, web, Error, HttpResponse};
use crate::model::{Product, CreateProductRequest};
use crate::startup::AppState;
use futures_util::StreamExt;
use crate::localwasmtime::validate_product;

pub async fn add_product(
    data: web::Data<AppState>,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    let mut products = data.products.lock().unwrap();
    let new_id = products.len() as i32 + 1;

    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > data.settings.max_size {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // Deserialize into CreateProductRequest first (without ID)
    let create_request = serde_json::from_slice::<CreateProductRequest>(&body)?;

    // Create full Product with generated ID and defaults
    let mut product = Product {
        id: new_id,
        name: create_request.name,
        brand: create_request.brand,
        price: create_request.price,
        description: create_request.description,
        category: create_request.category,
        sku: create_request.sku,
        in_stock: create_request.quantity_available > 0,
        quantity_available: create_request.quantity_available,
        image_url: create_request.image_url,
        rating: 0.0, // New products start with 0 rating
        specifications: create_request.specifications,
    };

    // Validate with rules engine (might need update for Best Buy rules)
    match validate_product(&data.settings, &product) {
        Ok(validated_product) => {
            products.push(validated_product.clone());
            Ok(HttpResponse::Ok().json(validated_product))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().body(e.to_string()))
        }   
    }
}