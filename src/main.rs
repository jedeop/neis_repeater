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

#[derive(serde::Serialize)]
struct AppResponse<T: serde::Serialize> {
    result: bool,
    content: Option<T>,
    error: Option<AppResponseError>,
}
impl<T: serde::Serialize> AppResponse<T> {
    fn success(content: T) -> Self {
        AppResponse {
            result: true,
            content: Some(content),
            error: None,
        }
    }
    fn error(error: AppResponseError) -> Self {
        AppResponse {
            result: false,
            content: None,
            error: Some(error),
        }
    }
    fn error_with_message(message: &str) -> Self {
        AppResponse {
            result: false,
            content: None,
            error: Some(AppResponseError {
                code: 1500,
                message: format!("Something went wrong: {}", message),
            }),
        }
    }
}

#[derive(serde::Serialize)]
struct AppResponseError {
    code: u32,
    message: String,
}
