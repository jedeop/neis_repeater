mod meal;
mod neis;
mod subjects;
mod time_table;

use std::{env, net::SocketAddr};

use axum::{routing::get, Router};
use dotenv::dotenv;
use meal::meal;
use time_table::time_table;

const DEFAULT_PORT: u16 = 3000;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/meal", get(meal))
        .route("/time_table", get(time_table));

    let port = match env::var("PORT") {
        Ok(val) => val.parse::<u16>().unwrap_or(DEFAULT_PORT),
        Err(_) => DEFAULT_PORT,
    };

    let socket_addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));
    println!("listening on {}", socket_addr);
    axum::Server::bind(&socket_addr)
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
