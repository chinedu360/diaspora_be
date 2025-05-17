use actix_web::{ HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateItemRequest {
    description: String,
    weight: f64,
    dimensions: Dimensions,
    origin_country: String,
    destination_country: String,
    price: Option<f64>,
    // pickup_required: Option<bool>
}

#[derive(Deserialize)]
struct Dimensions {
    length: f64,
    width: f64,
    height: f64,
}

pub async fn create_item(_item: web::Json<CreateItemRequest>) -> HttpResponse {
    HttpResponse::Ok().finish()
}