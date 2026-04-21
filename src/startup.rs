use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use std::sync::Mutex;
use crate::routes::{
    health, add_product, get_products, get_product, delete_product, update_product,
    ai_health, ai_generate_description, ai_generate_image,
    get_products_by_category, check_inventory, update_inventory
};
use crate::configuration::Settings;
use crate::model::Product;
use crate::data::fetch_products;

pub struct AppState {
    pub products: Mutex<Vec<Product>>,
    pub settings: Settings,
}

pub fn run(settings: Settings) -> std::io::Result<actix_web::dev::Server> {
    // Remove the argument from fetch_products call
    let products = fetch_products();  // Changed: removed &settings
    
    let state = web::Data::new(AppState {
        products: Mutex::new(products),
        settings: settings.clone(),
    });

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(state.clone())
            // Health check routes
            .route("/health", web::get().to(health))
            .route("/ai/health", web::get().to(ai_health))
            
            // AI routes
            .route("/ai/generate/description", web::post().to(ai_generate_description))
            .route("/ai/generate/image", web::post().to(ai_generate_image))
            
            // Product CRUD routes
            .route("/products", web::get().to(get_products))
            .route("/products", web::post().to(add_product))
            .route("/products/{product_id}", web::get().to(get_product))
            .route("/products/{product_id}", web::put().to(update_product))
            .route("/products/{product_id}", web::delete().to(delete_product))
            
            // Best Buy specific routes
            .route("/products/category/{category}", web::get().to(get_products_by_category))
            .route("/products/{product_id}/inventory", web::get().to(check_inventory))
            .route("/products/{product_id}/inventory", web::put().to(update_inventory))
    })
    .bind((settings.host.as_str(), settings.port))?
    .run();

    Ok(server)
}