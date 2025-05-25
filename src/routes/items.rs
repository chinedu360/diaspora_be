use actix_web::{web, HttpResponse};
use chrono::Utc;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct CreateItemRequest {
    description: String,
    weight: f64,
    dimensions: Dimensions,
    origin_country: String,
    destination_country: String,
    price: Option<Decimal>,
    pickup_required: Option<bool>,
}

#[derive(Deserialize, Serialize)]
#[allow(dead_code)]
struct Dimensions {
    length: f64,
    width: f64,
    height: f64,
}

#[tracing::instrument(
    name = "Adding a new item",
    skip(item, pool),
    fields(
        description = %item.description,
        price = ?item.price,
        origin_country = %item.origin_country
    )
)]
pub async fn create_item(
    item: web::Json<CreateItemRequest>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    tracing::info!("Processing new item creation request");

    match insert_item(&pool, &item).await {
        Ok(_) => {
            tracing::info!("New item has been saved to the database");
            HttpResponse::Created().finish()
        }
        Err(e) => {
            tracing::error!("Failed to save item: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Saving new items in thr database", skip(pool, item))]
async fn insert_item(pool: &PgPool, item: &CreateItemRequest) -> Result<(), sqlx::Error> {
    let dimensions_json =
        serde_json::to_value(&item.dimensions).expect("Failed to serialize dimensions");

    let default_price = Decimal::new(0, 0);

    sqlx::query!(
        r#"
        INSERT INTO items (
            id,
            description, 
            weight, 
            dimensions, 
            origin_country, 
            destination_country, 
            price,
            pickup_required,
            status,
            created_at,
            updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
        Uuid::new_v4(),                        // $1
        item.description,                      // $2
        item.weight,                           // $3
        dimensions_json,                       // $4
        item.origin_country,                   // $5
        item.destination_country,              // $6
        item.price.unwrap_or(default_price),   // $7
        item.pickup_required.unwrap_or(false), // $8
        "pending",                             // $9
        Utc::now(),                            // $10
        Utc::now()                             // $11 (removed trailing comma)
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to executes query: {:?}", e);
        e
    })?;

    Ok(())
}
