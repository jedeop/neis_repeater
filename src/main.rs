mod meal;
mod neis;
mod timetable;

use axum::{routing::get, Router};
use dotenv::dotenv;
use meal::meal;
use timetable::time_table;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/meal", get(meal))
        .route("/timetable", get(time_table));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
