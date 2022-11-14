use std::env;

use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use reqwest::StatusCode;

use crate::{
    neis::{
        meal::{MealData, MealType},
        NeisClient,
    },
    AppResponse,
};

pub(crate) async fn meal(Query(params): Query<MealQuery>) -> Response {
    let neis_client = NeisClient::new(&env::var("NEIS_API_KEY").expect("NEIS_API_KEY env missing"));

    let date = match params.date {
        Some(date) => date,
        None => Utc::now().naive_local().format("%Y%m%d").to_string(),
    };

    let res = neis_client
        .meal(
            &params.region_code,
            &params.school_code,
            &params.meal_type.unwrap_or(MealType::All),
            &date,
        )
        .await;

    match res {
        Ok(data) => {
            let res = match data.result() {
                Some(meal) => AppResponse::success(Meal::from_meal_data(meal)),
                None => AppResponse::error(data.status().to_app_response_error()),
            };
            Json(res).into_response()
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AppResponse::<Vec<Meal>>::error_with_message(
                &err.to_string(),
            )),
        )
            .into_response(),
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct MealQuery {
    region_code: String,
    school_code: String,
    date: Option<String>,
    meal_type: Option<MealType>,
}

#[derive(serde::Serialize)]
struct Meal {
    meal_type: MealType,
    date: String,
    dish_names: Vec<String>,
}

impl Meal {
    fn from_meal_data(meal: &[MealData]) -> Vec<Meal> {
        meal.iter()
            .map(|meal| Meal {
                meal_type: meal.meal_type(),
                date: meal.date().to_string(),
                dish_names: meal.dish_names(),
            })
            .collect()
    }
}
