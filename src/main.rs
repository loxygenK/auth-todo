use api::start_api;
use sea_orm::Database;

use crate::api::ctx::Context;

mod api;
mod entity;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let db = Database::connect(std::env::var("DATABASE_URL").expect("DATABASE_URL to be served")).await?;

    let context = Context::new(db);

    start_api(context).await
}
