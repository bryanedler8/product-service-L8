use crate::model::Product;

// Add the missing fetch_products function
pub fn fetch_products() -> Vec<Product> {
    get_sample_products()
}

// Sample data for Best Buy electronics store
pub fn get_sample_products() -> Vec<Product> {
    vec![
        // Laptops
        Product {
            id: 1,
            name: "MacBook Pro 14-inch".to_string(),
            brand: "Apple".to_string(),
            price: 1999.99,
            description: "Powerful laptop with M3 chip for developers".to_string(),
            category: "Laptops".to_string(),
            sku: "APL-MBP14-M3-16-512".to_string(),
            in_stock: true,
            quantity_available: 25,
            image_url: "/macbook-pro-14.jpg".to_string(),
            rating: 4.8,
            specifications: serde_json::json!({
                "processor": "Apple M3",
                "ram": "16GB",
                "storage": "512GB SSD",
                "display": "14-inch Liquid Retina"
            }),
        },
        Product {
            id: 2,
            name: "Dell XPS 15".to_string(),
            brand: "Dell".to_string(),
            price: 1799.99,
            description: "Premium Windows laptop with stunning OLED display".to_string(),
            category: "Laptops".to_string(),
            sku: "DEL-XPS15-13-512".to_string(),
            in_stock: true,
            quantity_available: 15,
            image_url: "/dell-xps-15.jpg".to_string(),
            rating: 4.6,
            specifications: serde_json::json!({
                "processor": "Intel Core i7-13700H",
                "ram": "16GB",
                "storage": "512GB SSD",
                "display": "15.6-inch OLED"
            }),
        },
        // Smartphones
        Product {
            id: 3,
            name: "Samsung Galaxy S24 Ultra".to_string(),
            brand: "Samsung".to_string(),
            price: 1299.99,
            description: "Premium Android smartphone with AI features".to_string(),
            category: "Smartphones".to_string(),
            sku: "SAM-S24U-256-12".to_string(),
            in_stock: true,
            quantity_available: 50,
            image_url: "/galaxy-s24-ultra.jpg".to_string(),
            rating: 4.7,
            specifications: serde_json::json!({
                "processor": "Snapdragon 8 Gen 3",
                "ram": "12GB",
                "storage": "256GB",
                "display": "6.8-inch Dynamic AMOLED"
            }),
        },
        // Audio
        Product {
            id: 4,
            name: "Sony WH-1000XM5".to_string(),
            brand: "Sony".to_string(),
            price: 399.99,
            description: "Industry-leading noise canceling headphones".to_string(),
            category: "Audio".to_string(),
            sku: "SNY-WH1000XM5-BLK".to_string(),
            in_stock: true,
            quantity_available: 100,
            image_url: "/sony-wh1000xm5.jpg".to_string(),
            rating: 4.8,
            specifications: serde_json::json!({
                "type": "Over-ear",
                "noise_canceling": true,
                "battery_life": "30 hours",
                "wireless": true
            }),
        },
        // Gaming
        Product {
            id: 5,
            name: "PlayStation 5 Slim".to_string(),
            brand: "Sony".to_string(),
            price: 499.99,
            description: "Next-gen gaming console with ultra-fast SSD".to_string(),
            category: "Gaming".to_string(),
            sku: "SNY-PS5-SLIM-1TB".to_string(),
            in_stock: true,
            quantity_available: 15,
            image_url: "/ps5-slim.jpg".to_string(),
            rating: 4.9,
            specifications: serde_json::json!({
                "storage": "1TB SSD",
                "resolution": "4K",
                "refresh_rate": "120Hz",
                "ray_tracing": true
            }),
        },
        // TVs
        Product {
            id: 6,
            name: "LG C3 OLED 65-inch".to_string(),
            brand: "LG".to_string(),
            price: 2499.99,
            description: "Stunning OLED TV with perfect blacks".to_string(),
            category: "TVs".to_string(),
            sku: "LG-C3-65-OLED".to_string(),
            in_stock: true,
            quantity_available: 10,
            image_url: "/lg-c3-oled.jpg".to_string(),
            rating: 4.9,
            specifications: serde_json::json!({
                "size": "65-inch",
                "display_type": "OLED evo",
                "resolution": "4K",
                "refresh_rate": "120Hz"
            }),
        },
    ]
}